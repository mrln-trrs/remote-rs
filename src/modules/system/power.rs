use crate::modules::native;

pub fn shutdown(_force: bool) -> bool {
    unsafe { native::SystemAction(1) }
}

pub fn reboot(_force: bool) -> bool {
    unsafe { native::SystemAction(2) }
}

pub fn sleep() -> bool {
    unsafe { native::SystemAction(3) }
}
