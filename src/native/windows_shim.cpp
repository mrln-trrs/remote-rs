#include <windows.h>
#include <mmdeviceapi.h>
#include <endpointvolume.h>
#include <powrprof.h>
#include "windows_shim.h"

#pragma comment(lib, "user32.lib")
#pragma comment(lib, "ole32.lib")
#pragma comment(lib, "PowrProf.lib")

// ... (Anteriores funciones GetMasterVolume, SetMasterVolume, LockSystem, SystemAction se mantienen igual) ...

// Re-añadiendo todo para asegurar integridad
float GetMasterVolume() {
    float currentVolume = 0;
    CoInitialize(NULL);
    IMMDeviceEnumerator *deviceEnumerator = NULL;
    CoCreateInstance(__uuidof(MMDeviceEnumerator), NULL, CLSCTX_INPROC_SERVER, __uuidof(IMMDeviceEnumerator), (LPVOID *)&deviceEnumerator);
    if (deviceEnumerator) {
        IMMDevice *defaultDevice = NULL;
        deviceEnumerator->GetDefaultAudioEndpoint(eRender, eMultimedia, &defaultDevice);
        if (defaultDevice) {
            IAudioEndpointVolume *endpointVolume = NULL;
            defaultDevice->Activate(__uuidof(IAudioEndpointVolume), CLSCTX_INPROC_SERVER, NULL, (LPVOID *)&endpointVolume);
            if (endpointVolume) {
                endpointVolume->GetMasterVolumeLevelScalar(&currentVolume);
                endpointVolume->Release();
            }
            defaultDevice->Release();
        }
        deviceEnumerator->Release();
    }
    CoUninitialize();
    return currentVolume;
}

void SetMasterVolume(float level) {
    CoInitialize(NULL);
    IMMDeviceEnumerator *deviceEnumerator = NULL;
    CoCreateInstance(__uuidof(MMDeviceEnumerator), NULL, CLSCTX_INPROC_SERVER, __uuidof(IMMDeviceEnumerator), (LPVOID *)&deviceEnumerator);
    if (deviceEnumerator) {
        IMMDevice *defaultDevice = NULL;
        deviceEnumerator->GetDefaultAudioEndpoint(eRender, eMultimedia, &defaultDevice);
        if (defaultDevice) {
            IAudioEndpointVolume *endpointVolume = NULL;
            defaultDevice->Activate(__uuidof(IAudioEndpointVolume), CLSCTX_INPROC_SERVER, NULL, (LPVOID *)&endpointVolume);
            if (endpointVolume) {
                endpointVolume->SetMasterVolumeLevelScalar(level, NULL);
                endpointVolume->Release();
            }
            defaultDevice->Release();
        }
        deviceEnumerator->Release();
    }
    CoUninitialize();
}

bool LockSystem() { return LockWorkStation(); }

bool SystemAction(int action) {
    switch (action) {
        case 1: return ExitWindowsEx(EWX_SHUTDOWN | EWX_FORCE, SHTDN_REASON_MAJOR_OTHER);
        case 2: return ExitWindowsEx(EWX_REBOOT | EWX_FORCE, SHTDN_REASON_MAJOR_OTHER);
        case 3: return SetSuspendState(FALSE, FALSE, FALSE);
        default: return false;
    }
}

// NUEVAS FUNCIONES OPTIMIZADAS
BatteryInfo GetBatteryStatus() {
    SYSTEM_POWER_STATUS sps;
    BatteryInfo info = { -1, false };
    if (GetSystemPowerStatus(&sps)) {
        info.percentage = (int)sps.BatteryLifePercent;
        info.is_charging = (sps.ACLineStatus == 1);
    }
    return info;
}

void MoveMouse(int dx, int dy) {
    INPUT input = {0};
    input.type = INPUT_MOUSE;
    input.mi.dwFlags = MOUSEEVENTF_MOVE;
    input.mi.dx = dx;
    input.mi.dy = dy;
    SendInput(1, &input, sizeof(INPUT));
}

void ClickMouse(int button) {
    INPUT inputs[2] = {0};
    inputs[0].type = INPUT_MOUSE;
    inputs[1].type = INPUT_MOUSE;
    
    if (button == 0) { // Left
        inputs[0].mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
        inputs[1].mi.dwFlags = MOUSEEVENTF_LEFTUP;
    } else { // Right
        inputs[0].mi.dwFlags = MOUSEEVENTF_RIGHTDOWN;
        inputs[1].mi.dwFlags = MOUSEEVENTF_RIGHTUP;
    }
    SendInput(2, inputs, sizeof(INPUT));
}
