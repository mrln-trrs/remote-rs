# Remote-RS 🚀

Sistema de control remoto ligero para Windows vía Wi-Fi local.

## Stack
- **Backend:** Rust (Axum, Tokio) + C++ (Win32, CoreAudio).
- **Frontend:** Svelte 5 (Vite, TypeScript).

## Inicio Rápido

### Requisitos
- Rust (Cargo)
- Node.js & npm
- Visual Studio Build Tools (C++)

### Ejecución
1.  **Backend:** `cargo run` (Escucha en puerto 3000).
2.  **Frontend:** `cd frontend && npm install && npm run dev -- --host` (Escucha en puerto 5173).

## Documentación
- [Arquitectura](./documentation/architecture.md)
- [Guía de Desarrollo](./documentation/development_guide.md)
- [Investigación de APIs](./documentation/research_audio_system.md)
