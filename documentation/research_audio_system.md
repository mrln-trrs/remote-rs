# Research: Windows Volume Control

## Objective
Implement precise volume control (0-100%) rather than just simulating volume keys.

## API Options
1.  **Core Audio API (Win32):**
    - `IMMDeviceEnumerator`: To find the default audio endpoint.
    - `IAudioEndpointVolume`: To get/set the master volume level.
    - Required features in `windows` crate: `Win32_Media_Audio`, `Win32_Media_Audio_Endpoints`, `Win32_Devices_Enumeration`, `Win32_System_Com`.

2.  **Simulation (Enigo):**
    - Simple but imprecise. Only supports "Up/Down" steps.

## Implementation Plan
We will use the Core Audio API for absolute volume control. This requires initializing COM (`CoInitializeEx`).

---

# Research: System Power & Security

## Objective
Allow basic power management: Lock, Sleep, Shutdown, Restart.

## API Options
- **Lock:** `LockWorkStation()` in `user32.dll`.
- **Sleep:** `SetSuspendState()` in `Powrprof.dll`.
- **Shutdown/Restart:** `ExitWindowsEx()` or `InitiateSystemShutdownEx()`.

## Implementation Plan
Create a `src/modules/system/` directory with individual files for each action.
- `lock.rs`
- `power.rs` (Shutdown/Restart/Sleep)
