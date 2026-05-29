use crate::modules::native;

pub fn get_volume() -> f32 {
    unsafe { native::GetMasterVolume() }
}

pub fn set_volume(level: f32) {
    unsafe { native::SetMasterVolume(level) }
}
