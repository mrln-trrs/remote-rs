mod modules;

use axum::{
    routing::{get, post},
    Router, Json, extract::Path,
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
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
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
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

async fn api_terminal_exec(Json(payload): Json<serde_json::Value>) -> String {
    if let Some(cmd) = payload["command"].as_str() {
        modules::terminal::execute_single_command(cmd)
    } else {
        "No command provided".to_string()
    }
}
