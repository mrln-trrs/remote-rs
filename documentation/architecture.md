# Architecture: Remote-RS

## Project Overview
A lightweight remote control system for Windows, allowing control from a mobile device via a local Wi-Fi network.

## Tech Stack
- **Backend:** Rust (Axum, Tokio, Enigo) + **Native C++ Shim** (Core Audio, System APIs)
- **Frontend:** Svelte 5 (Vite, TypeScript)
- **Communication:** REST API (JSON) + Future WebSockets for low-latency tasks.

## Modular Backend Structure
- `src/native/`: C++ code for low-level Windows API interaction (Volume, Power).
- `src/modules/native.rs`: FFI declarations for the C++ shim.
- `src/modules/media/`: Handles GSMTC (Currently investigating Rust vs C++ implementation).

## Technical Challenges & Research
### Windows API Integration
The `windows` crate (v0.58/v0.60) provides access to WinRT and Win32 APIs.
- **Status:** **Hybrid approach implemented.** Volume and System Power controls are now handled via a native C++ shim (`src/native/windows_shim.cpp`) linked to Rust via FFI. This provides 100% reliable access to Win32 and Core Audio APIs.
- **Next:** Refactor Media Metadata (GSMTC) into the C++ shim to complete the native transition.

## API Specification
### Media Module
- `GET /api/media`: Returns current title, artist, and playback status.
- `POST /api/control/playpause`: Toggles playback.
- `POST /api/control/next`: Next track.
- `POST /api/control/prev`: Previous track.

### Volume Module
- `POST /api/control/volume/up`: Increase system volume.
- `POST /api/control/volume/down`: Decrease system volume.
- `POST /api/volume/{level}`: Set absolute volume level (0-100).
