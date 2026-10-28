/// # Safety
///
/// Do not call this function if disable is not called
#[inline]
pub unsafe fn enable() {
    unsafe {
        core::arch::asm!("cpsie i", options(nostack, preserves_flags));
    }
}

#[inline]
pub fn disable() {
    unsafe {
        core::arch::asm!("cpsid i", options(nostack, preserves_flags));
    }
}
