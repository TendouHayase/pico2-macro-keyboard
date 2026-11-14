#[inline]
pub(crate) fn read() -> u32 {
    unsafe {
        let r: u32;
        core::arch::asm!("mrs {}, IPSR",  out(reg) r);
        r
    }
}
