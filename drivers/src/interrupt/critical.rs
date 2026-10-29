pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let restore_state = crate::register::primask::read();
    super::disable();

    let ret = f();

    if restore_state.is_active() {
        unsafe { super::enable() };
    }

    ret
}
