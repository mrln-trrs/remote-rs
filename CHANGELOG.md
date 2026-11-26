# Registro de cambios

Todos los cambios relevantes de este proyecto se documentan en este archivo.

El formato sigue [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/) y el proyecto usa [Versionado Semántico](https://semver.org/lang/es/).

## [0.1.0] - 2026-11-26

### Añadido
- Servidor Axum con WebSocket de estado (multimedia y batería) y API para controles multimedia, volumen, bloqueo, apagado, reinicio, suspensión, ratón, apertura de URL y YouTube, y terminal PowerShell.
- Shim en C++ (Win32/CoreAudio) compilado con `cc`.
- Frontend móvil en Svelte 5 + Vite.
- Variables de entorno `REMOTE_RS_BIND`, `REMOTE_RS_ALLOWED_ORIGINS` y `REMOTE_RS_TERMINAL_TOKEN`.
- README, licencia MIT, política de seguridad y configuración de Dependabot para cargo y npm.

### Cambiado
- Dirección de escucha por defecto: `127.0.0.1:3000` (antes `0.0.0.0:3000`).
- CORS restringido a los orígenes configurados (antes permisivo).

### Seguridad
- La terminal remota queda desactivada por defecto; si se activa, exige un token en la cabecera `X-Remote-Token` (comparación en tiempo constante).
- `quinn-proto` 0.11.19 y `rustls` 0.23.46 (avisos de Dependabot).
- Frontend: `vite` ≥ 8.1.6, `npm update` (devalue, postcss, source-map-js) y `path-to-regexp` ≥ 1.9.0 forzado con `overrides`.

[0.1.0]: https://github.com/mrln-trrs/remote-rs/releases/tag/v0.1.0
