# Research: Performance & Battery Optimization

## Objective
Reduce laptop resource consumption (CPU, Network, Battery) to near-zero when idle.

## 1. Architecture Shift: WebSockets
Currently, the frontend polls `/api/media` every 2 seconds.
- **Problem:** Constant HTTP handshakes and parsing, even if nothing changes.
- **Solution:** Implement WebSockets (via `axum` and `tokio-tungstenite`).
    - The backend only pushes data when a Windows Event is triggered (e.g., track change, volume change).
    - CPU usage drops because the backend stays "asleep" until an event occurs.

## 2. Native Optimization
- **Binary Size:** Use `lto = true` and `opt-level = "z"` in `Cargo.toml`.
- **Memory:** Avoid string allocations where possible. Use the C++ shim for all "heavy" Windows API work to avoid WinRT overhead in Rust.

---

# New Lightweight Features (Mobile Control)

## 1. Battery & Power Status
- **Why:** Essential for a remote control.
- **API:** `GetSystemPowerStatus` (Win32).
- **Impact:** Negligible. One call every 30s or on request.

## 2. Mouse/Touchpad Mode
- **Why:** Control the laptop as if the phone were a trackpad.
- **API:** `mouse_event` (Win32) or `Enigo`.
- **Impact:** Requires WebSocket for low latency.

## 3. Basic Process Control
- **Why:** Close a stuck app or alt-tab from the phone.
- **API:** `TerminateProcess`, `SetForegroundWindow`.
```
