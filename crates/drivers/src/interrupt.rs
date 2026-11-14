mod critical;
mod irq;
mod table;

pub use critical::*;
pub use irq::*;
pub use table::*;

/// # Safety
///
/// Do not call this function if disable is not called
#[inline]
unsafe fn enable() {
    unsafe {
        core::arch::asm!("cpsie i", options(nostack, preserves_flags));
    }
}

#[inline]
fn disable() {
    unsafe {
        core::arch::asm!("cpsid i", options(nostack, preserves_flags));
    }
}
