#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Primask {
    Active,
    Inactive,
}

impl Primask {
    pub fn is_active(&self) -> bool {
        *self == Primask::Active
    }
}

#[inline]
pub(crate) fn read() -> Primask {
    unsafe {
        let r: u32;
        core::arch::asm!("mrs {}, PRIMASK",  out(reg) r);
        if r & (1 << 0) == (1 << 0) {
            Primask::Inactive
        } else {
            Primask::Active
        }
    }
}

#[inline]
pub(crate) fn write(primask: Primask) {
    unsafe {
        match primask {
            Primask::Inactive => core::arch::asm!("msr PRIMASK, r1", in("r1") 0x01),
            Primask::Active => core::arch::asm!("msr PRIMASK, r1", in("r1") 0x00),
        }
    }
}
