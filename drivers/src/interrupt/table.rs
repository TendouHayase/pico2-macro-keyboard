use core::cell::UnsafeCell;

use crate::interrupt::irq::{IRQ_COUNT, Irq};

pub struct IrqHandlerTable {
    table: UnsafeCell<[Option<&'static mut (dyn FnMut() + Send)>; IRQ_COUNT]>,
}

pub static IRQ_HANDLER_TABLE: IrqHandlerTable = IrqHandlerTable::new();

impl IrqHandlerTable {
    const fn new() -> Self {
        Self {
            table: UnsafeCell::new([const { None }; IRQ_COUNT]),
        }
    }

    /// # Safety
    /// - Must not be called for `irq` while the handler registered for `irq` is executing
    ///   (e.g. from within that handler via [`Self::execute`]).
    /// - Must not be called from an NMI or HardFault handler, or from core 1.
    pub unsafe fn insert(
        &self,
        irq: Irq,
        handler: &'static mut (dyn FnMut() + Send),
    ) -> Option<&'static mut (dyn FnMut() + Send)> {
        super::free(move || unsafe { (*self.table.get())[irq as usize].replace(handler) })
    }

    /// # Safety
    /// - Must not be called for `irq` while the handler registered for `irq` is executing
    ///   (e.g. from within that handler via [`Self::execute`]).
    /// - Must not be called from an NMI or HardFault handler, or from core 1.
    pub unsafe fn remove(&self, irq: Irq) -> Option<&'static mut (dyn FnMut() + Send)> {
        super::free(|| unsafe { (*self.table.get())[irq as usize].take() })
    }

    /// Runs the handler registered for `irq` with interrupts masked; handlers are not
    /// preemptible by other interrupts.
    ///
    /// # Safety
    /// - Must not be called for `irq` while the handler for `irq` is already executing
    ///   (e.g. re-entrantly from within that handler).
    /// - Must not be called from an NMI or HardFault handler, or from core 1.
    pub unsafe fn execute(&self, irq: Irq) {
        super::free(|| unsafe {
            if let Some(ref mut closure) = (*self.table.get())[irq as usize] {
                closure();
            }
        })
    }
}

// SAFETY: Every access to the inner `UnsafeCell` occurs inside `free`, which masks
// preemption on the current core. The safety contracts of `insert`, `remove`, and
// `execute` forbid calls from NMI/HardFault handlers and from core 1, and exclude
// same-slot re-entrancy.
unsafe impl Sync for IrqHandlerTable {}

/// Common entry point for every IRQ vector entry.
///
/// # Safety
/// Must only be invoked by hardware as an IRQ vector entry (exception number 16 or
/// above) on core 0. It must never be called directly from software.
#[unsafe(no_mangle)]
unsafe extern "C" fn dispatch() {
    let ipsr = crate::register::ipsr::read();

    let Some(irq) = ipsr
        .checked_sub(16)
        .and_then(|n| u8::try_from(n).ok())
        .and_then(|n| Irq::try_from(n).ok())
    else {
        panic!("dispatch: invoked outside an IRQ vector entry");
    };

    unsafe { IRQ_HANDLER_TABLE.execute(irq) };
}
