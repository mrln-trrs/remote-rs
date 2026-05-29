use serde::Serialize;

#[repr(C)]
#[derive(Serialize, Clone, Copy)]
pub struct BatteryInfo {
    pub percentage: i32,
    pub is_charging: bool,
}

unsafe extern "C" {
    pub fn GetMasterVolume() -> f32;
    pub fn SetMasterVolume(level: f32);
    pub fn LockSystem() -> bool;
    pub fn SystemAction(action: i32) -> bool;
    
    pub fn GetBatteryStatus() -> BatteryInfo;
    pub fn MoveMouse(dx: i32, dy: i32);
    pub fn ClickMouse(button: i32);
}
