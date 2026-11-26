mod modules;

use axum::{
    routing::{get, post},
    Router, Json, extract::Path,
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    http::{HeaderMap, HeaderValue, Method, StatusCode, header},
};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::sync::broadcast;
use tower_http::cors::{AllowOrigin, CorsLayer};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
use crate::modules::media::{metadata, controls};
use crate::modules::volume;
use crate::modules::system;

#[derive(Clone, serde::Serialize)]
struct AppState {
    media: metadata::MediaMetadata,
    battery: modules::native::BatteryInfo,
}

#[tokio::main]
async fn main() {
    unsafe { let _ = CoInitializeEx(None, COINIT_MULTITHREADED); }

    // Create a broadcast channel for real-time updates
    let (tx, _) = broadcast::channel::<String>(16);
    let tx_clone = tx.clone();

    // Background task to monitor state and broadcast changes
    tokio::spawn(async move {
        let mut last_state_json = String::new();
        loop {
            let mut media = metadata::get_current_media().await;
            media.volume = (volume::get_volume() * 100.0) as u32;
            let battery = unsafe { modules::native::GetBatteryStatus() };
            
            let current_state = AppState { media, battery };
            if let Ok(json) = serde_json::to_string(&current_state) {
                if json != last_state_json {
                    let _ = tx_clone.send(json.clone());
                    last_state_json = json;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }
    });

    let app = Router::new()
        .route("/ws", get(move |ws: WebSocketUpgrade| {
            let tx = tx.clone();
            async move { ws.on_upgrade(move |socket| handle_socket(socket, tx)) }
        }))
        .route("/api/media", get(api_get_media))
        .route("/api/control/{command}", post(api_handle_control))
        .route("/api/volume/{level}", post(api_set_volume))
        .route("/api/system/{action}", post(api_handle_system))
        .route("/api/system/battery", get(api_get_battery))
        .route("/api/mouse/move/{dx}/{dy}", post(api_move_mouse))
        .route("/api/mouse/click/{button}", post(api_click_mouse))
        .route("/api/launch/url", post(api_launch_url))
        .route("/api/launch/youtube", post(api_launch_youtube))
        .route("/api/search/youtube", get(api_search_youtube))
        .route("/api/launch/youtube/play/{id}", post(api_play_youtube_video))
        .route("/api/terminal/exec", post(api_terminal_exec))
        .layer(cors_layer());

    // Por defecto solo escucha en el propio equipo. Para usarlo desde el celular
    // hay que indicar la interfaz de red de forma explícita (ver README).
    let addr: SocketAddr = std::env::var("REMOTE_RS_BIND")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 3000)));
    if terminal_token().is_some() {
        println!("Terminal remota ACTIVADA (protegida con REMOTE_RS_TERMINAL_TOKEN)");
    } else {
        println!("Terminal remota desactivada (defina REMOTE_RS_TERMINAL_TOKEN para activarla)");
    }
    println!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<String>) {
    let mut rx = tx.subscribe();

    let initial_media = current_media_snapshot().await;
    let initial_state = AppState {
        media: initial_media,
        battery: unsafe { modules::native::GetBatteryStatus() },
    };

    if let Ok(json) = serde_json::to_string(&initial_state) {
        if socket.send(Message::Text(json.into())).await.is_err() {
            return;
        }
    }
    
    loop {
        tokio::select! {
            msg = rx.recv() => {
                if let Ok(text) = msg {
                    if socket.send(Message::Text(text.into())).await.is_err() {
                        break;
                    }
                }
            }
            msg = socket.recv() => {
                if msg.is_none() { break; }
                // Handle incoming messages from mobile (e.g. touchpad movements)
            }
        }
    }
}

async fn api_get_media() -> Json<metadata::MediaMetadata> {
    Json(current_media_snapshot().await)
}

async fn current_media_snapshot() -> metadata::MediaMetadata {
    let mut meta = metadata::get_current_media().await;
    meta.volume = (volume::get_volume() * 100.0) as u32;
    meta
}

async fn api_handle_control(Path(command): Path<String>) {
    controls::send_media_key(&command);
}

async fn api_set_volume(Path(level): Path<f32>) {
    volume::set_volume(level / 100.0);
}

async fn api_handle_system(Path(action): Path<String>) {
    match action.as_str() {
        "lock" => { system::lock::lock_pc(); },
        "shutdown" => { system::power::shutdown(false); },
        "reboot" => { system::power::reboot(false); },
        "sleep" => { system::power::sleep(); },
        _ => {}
    }
}

async fn api_get_battery() -> Json<modules::native::BatteryInfo> {
    unsafe { Json(modules::native::GetBatteryStatus()) }
}

async fn api_move_mouse(Path((dx, dy)): Path<(i32, i32)>) {
    unsafe { modules::native::MoveMouse(dx, dy); }
}

async fn api_click_mouse(Path(button): Path<i32>) {
    unsafe { modules::native::ClickMouse(button); }
}

async fn api_launch_url(Json(payload): Json<serde_json::Value>) {
    if let Some(url) = payload["url"].as_str() {
        modules::launcher::open_url(url);
    }
}

async fn api_launch_youtube(Json(payload): Json<serde_json::Value>) {
    if let Some(query) = payload["query"].as_str() {
        modules::launcher::search_youtube(query);
    }
}

async fn api_search_youtube(axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>) -> Json<Vec<modules::launcher::youtube::YouTubeResult>> {
    let query = params.get("q").cloned().unwrap_or_default();
    Json(modules::launcher::youtube::search_youtube_results(&query).await)
}

async fn api_play_youtube_video(Path(id): Path<String>) {
    modules::launcher::youtube::play_video(&id);
}

/// CORS restringido: solo los orígenes de `REMOTE_RS_ALLOWED_ORIGINS`
/// (separados por comas) o, si no se define, el frontend local en el puerto 5173.
fn cors_layer() -> CorsLayer {
    let origins: Vec<HeaderValue> = std::env::var("REMOTE_RS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173,http://127.0.0.1:5173".to_string())
        .split(',')
        .filter_map(|o| HeaderValue::from_str(o.trim()).ok())
        .collect();
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::HeaderName::from_static("x-remote-token")])
}

/// Token de la terminal remota. Sin token (o vacío) la terminal queda desactivada.
fn terminal_token() -> Option<String> {
    std::env::var("REMOTE_RS_TERMINAL_TOKEN")
        .ok()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

/// Comparación en tiempo constante para no filtrar el token por tiempos de respuesta.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

async fn api_terminal_exec(headers: HeaderMap, Json(payload): Json<serde_json::Value>) -> (StatusCode, String) {
    let Some(expected) = terminal_token() else {
        return (StatusCode::FORBIDDEN, "Terminal remota desactivada".to_string());
    };
    let provided = headers
        .get("x-remote-token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !constant_time_eq(provided.as_bytes(), expected.as_bytes()) {
        return (StatusCode::UNAUTHORIZED, "Token no válido".to_string());
    }
    if let Some(cmd) = payload["command"].as_str() {
        (StatusCode::OK, modules::terminal::execute_single_command(cmd))
    } else {
        (StatusCode::BAD_REQUEST, "No command provided".to_string())
    }
}
