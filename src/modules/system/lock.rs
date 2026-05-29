use crate::modules::native;

pub fn lock_pc() -> bool {
    unsafe { native::LockSystem() }
}
