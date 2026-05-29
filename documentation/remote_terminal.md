# Research & Safety: Remote Terminal Access

## ⚠️ SECURITY WARNING
Enabling a remote terminal gives **full control** over the laptop. 
- Only use this on a **trusted local Wi-Fi**.
- Do not expose this port to the internet.
- Future enhancement: Add a PIN/Password for this specific module.

## Technical Strategy: Pseudo-Terminal (PTY)
To have a "real" terminal experience (where you can see the prompt, use interactive commands, etc.), we need more than just running `std::process::Command`.

### 1. The Communication Layer (WebSockets)
HTTP is too slow for a terminal. We need a persistent bidirectional connection.
- **Backend:** Axum + `tokio-tungstenite`.
- **Frontend:** A terminal emulator component like `xterm.js`.

### 2. The PTY Layer (Windows)
Windows uses **ConPTY** (introduced in Windows 10).
- We need to create a "Pseudo Console".
- Redirect the input/output streams of a process (PowerShell or CMD) to our WebSocket.

### 3. Implementation Plan
- **Library:** Use `portable-pty` or `conpty` crate to handle the complex Windows handle redirection.
- **Module:** `src/modules/terminal/`.
- **Default Shell:** PowerShell 7 (if available) or standard PowerShell.

## Mobile UI Considerations
- Typing on a mobile phone for a terminal is hard.
- **Optimization:** Add "Quick Buttons" in the UI for common commands (e.g., `ipconfig`, `dir`, `tasklist`, `npm run dev`).
