# Remote-RS

Control remoto ligero de un PC con Windows desde el celular a través de la red Wi-Fi local.

![Estado](https://img.shields.io/badge/estado-experimental-orange)
![Licencia](https://img.shields.io/badge/licencia-MIT-blue)
![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)
![Svelte](https://img.shields.io/badge/Svelte-5-ff3e00?logo=svelte)

> [!WARNING]
> **Advertencia de seguridad.** Remote-RS puede apagar el equipo, mover el ratón, abrir URL y, si se activa, **ejecutar comandos de PowerShell**. Úselo solo en una red local de confianza y **nunca exponga el puerto a Internet**.
>
> - Por defecto solo escucha en `127.0.0.1` (el propio equipo). Para usarlo desde el celular hay que abrirlo a la red de forma explícita con `REMOTE_RS_BIND`.
> - La terminal remota está **desactivada** por defecto. Solo se activa definiendo `REMOTE_RS_TERMINAL_TOKEN`, y entonces cada petición debe llevar ese token en la cabecera `X-Remote-Token`.
> - El resto de la API (multimedia, volumen, ratón, bloqueo, apagado, abrir URL) **no tiene autenticación**: cualquier dispositivo que alcance el puerto puede usarla.
> - El CORS restringido impide que otras páginas web lean las respuestas, pero no evita que envíen peticiones `POST` simples (por ejemplo, apagar el equipo) mientras el servidor esté accesible.

## Estado del proyecto

**Experimental** (v0.1.0). Funciona en Windows 10/11; la búsqueda de YouTube devuelve un resultado fijo de ejemplo y la terminal es de comandos sueltos (sin sesión interactiva).

## Características

- Estado en tiempo real por WebSocket (`/ws`): título, artista, miniatura, progreso y volumen de lo que suena, y batería.
- Controles multimedia (reproducir/pausar, siguiente, anterior, subir y bajar volumen) y volumen maestro.
- Sistema: bloquear, apagar, reiniciar y suspender.
- Ratón: movimiento relativo y clic.
- Abrir una URL o una búsqueda de YouTube en el navegador del PC.
- Terminal remota opcional: ejecuta un comando de PowerShell y devuelve su salida (`POST /api/terminal/exec`).
- Frontend móvil en Svelte 5 con páginas de inicio, reproductor, controles y sistema.

## Stack

| Parte | Tecnología |
|---|---|
| Backend | Rust (edición 2024, requiere ≥ 1.85), axum 0.8 (WebSocket), tokio 1, tower-http 0.6 (CORS), enigo 0.3, windows 0.58 |
| Código nativo | Shim en C++ (Win32, CoreAudio, PowrProf) compilado con `cc` en `build.rs` |
| Frontend | Svelte 5, Vite 8, TypeScript 6, `@lucide/svelte`, router `page` |

## Requisitos

- Windows 10/11 (el backend solo compila para Windows).
- Rust estable ≥ 1.85 y Visual Studio Build Tools con el compilador de C++.
- Node.js 22 y npm para el frontend.

## Configuración

| Variable | Uso | Por defecto |
|---|---|---|
| `REMOTE_RS_BIND` | Dirección y puerto de escucha | `127.0.0.1:3000` |
| `REMOTE_RS_ALLOWED_ORIGINS` | Orígenes permitidos por CORS, separados por comas | `http://localhost:5173,http://127.0.0.1:5173` |
| `REMOTE_RS_TERMINAL_TOKEN` | Activa la terminal remota y fija su token | sin definir (terminal desactivada) |

## Instalación y uso

```powershell
git clone https://github.com/mrln-trrs/remote-rs.git
cd remote-rs

# Frontend
cd frontend
npm ci
npm run dev        # Vite en el puerto 5173 (escucha en 0.0.0.0)
```

Backend, solo en el propio equipo:

```powershell
cargo run --release
```

Backend accesible desde el celular (sustituya `192.168.1.50` por la IP del PC en la red local):

```powershell
$env:REMOTE_RS_BIND = "0.0.0.0:3000"
$env:REMOTE_RS_ALLOWED_ORIGINS = "http://192.168.1.50:5173"
cargo run --release
```

Desde el celular, abra `http://192.168.1.50:5173`. El frontend busca el backend en el mismo host, puerto 3000.

Activar la terminal remota (use un token largo y aleatorio):

```powershell
$env:REMOTE_RS_TERMINAL_TOKEN = "<token-largo-y-aleatorio>"
cargo run --release
```

```bash
curl -X POST http://127.0.0.1:3000/api/terminal/exec \
  -H "Content-Type: application/json" \
  -H "X-Remote-Token: <token-largo-y-aleatorio>" \
  -d '{"command":"Get-Date"}'
```

## Estructura del proyecto

```
src/
├── main.rs              # servidor axum, rutas, CORS y token de la terminal
├── modules/
│   ├── media/           # metadatos (WinRT) y teclas multimedia
│   ├── volume/          # volumen maestro
│   ├── system/          # bloqueo, apagado, reinicio y suspensión
│   ├── launcher/        # abrir URL y YouTube
│   ├── terminal/        # ejecución de comandos PowerShell
│   └── native.rs        # enlaces FFI al shim de C++
└── native/              # windows_shim.cpp / .h
build.rs                 # compila el shim y enlaza user32, ole32 y PowrProf
frontend/                # aplicación móvil Svelte + Vite
documentation/           # arquitectura, guía de desarrollo e investigación
```

## Limitaciones conocidas

- Solo compila y funciona en Windows.
- Sin autenticación en la API salvo la terminal (ver la advertencia de seguridad). Un PIN o emparejamiento para toda la API queda pendiente.
- La búsqueda de YouTube (`/api/search/youtube`) devuelve un resultado fijo de ejemplo; no consulta YouTube.
- La terminal ejecuta comandos sueltos, sin sesión interactiva; el frontend aún no la usa.
- Conexión HTTP sin cifrar dentro de la red local.
- Dependencias con versiones mayores pendientes: windows 0.58 → 0.62, enigo 0.3 → 0.6 (cambian de API).
- Dependencias declaradas sin uso actual: `reqwest`, `portable-pty` y `windows-future`.
- Sin integración continua (requeriría un runner `windows-latest`).

## Documentación

- [Arquitectura](documentation/architecture.md)
- [Guía de desarrollo](documentation/development_guide.md)
- [Terminal remota](documentation/remote_terminal.md)
- [Investigación del sistema de audio](documentation/research_audio_system.md)

## Autor

Marlon Torres Espinoza ([@mrln-trrs](https://github.com/mrln-trrs)).

## Licencia

Distribuido bajo la licencia MIT. Ver [LICENSE](LICENSE).
