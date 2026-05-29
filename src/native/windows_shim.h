#ifndef WINDOWS_SHIM_H
#define WINDOWS_SHIM_H

#ifdef __cplusplus
extern "C" {
#endif

// Power & Audio
float GetMasterVolume();
void SetMasterVolume(float level);
bool LockSystem();
bool SystemAction(int action); // 1: shutdown, 2: reboot, 3: sleep

// New: Battery & Mouse
struct BatteryInfo {
    int percentage;
    bool is_charging;
};
BatteryInfo GetBatteryStatus();
void MoveMouse(int dx, int dy);
void ClickMouse(int button); // 0: left, 1: right

#ifdef __cplusplus
}
#endif

#endif
