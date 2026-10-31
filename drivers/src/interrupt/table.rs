use core::cell::UnsafeCell;

use crate::interrupt::irq::{IRQ_COUNT, Irq};

/// Handler table indexed by IRQ number, with one slot per [`Irq`].
///
/// Every access to a slot happens inside a critical section. Only core 0 is
/// supported.
pub struct IrqHandlerTable {
    table: UnsafeCell<[Option<fn()>; IRQ_COUNT]>,
}

/// The single table consulted by [`dispatch`].
pub static IRQ_HANDLER_TABLE: IrqHandlerTable = IrqHandlerTable::new();

impl IrqHandlerTable {
    const fn new() -> Self {
        Self {
            table: UnsafeCell::new([const { None }; IRQ_COUNT]),
        }
    }

    /// Registers `handler` for `irq` and returns the previously registered handler.
    ///
    /// May be called from any handler, including the one registered for `irq`.
    ///
    /// # Safety
    /// - Must not be called from an NMI or HardFault handler, which can preempt code
    ///   even inside a critical section.
    /// - Must only be called on core 0. The critical section masks only the current
    ///   core.
    pub unsafe fn insert(&self, irq: Irq, handler: fn()) -> Option<fn()> {
        unsafe { super::free(|| (*self.table.get())[irq as usize].replace(handler)) }
    }

    /// Unregisters the handler for `irq` and returns it.
    ///
    /// # Safety
    /// Same requirements as [`Self::insert`].
    pub unsafe fn remove(&self, irq: Irq) -> Option<fn()> {
        unsafe { super::free(|| (*self.table.get())[irq as usize].take()) }
    }

    /// Calls the handler registered for `irq`, if any, with interrupts masked.
    ///
    /// The handler cannot be preempted by other IRQs, so NVIC priorities do not
    /// apply between handlers, and a long-running handler delays every other IRQ.
    /// The handler pointer is copied out of the slot before the call, so the handler
    /// may call [`Self::insert`] or [`Self::remove`].
    ///
    /// # Safety
    /// Same requirements as [`Self::insert`].
    pub unsafe fn execute(&self, irq: Irq) {
        unsafe {
            super::free(|| {
                if let Some(func) = (*self.table.get())[irq as usize] {
                    func();
                }
            })
        }
    }
}

// SAFETY: Every access to the inner `UnsafeCell` happens inside `free`, which masks
// interrupts on the current core, so accesses never overlap on core 0. The safety
// contracts forbid calls from NMI/HardFault handlers, which `free` cannot mask, and
// from core 1, which `free` does not affect.
unsafe impl Sync for IrqHandlerTable {}

/// Common handler installed in every IRQ entry of the vector table.
///
/// Reads the active exception number from IPSR, converts it to an [`Irq`], and runs
/// the handler registered for it. Panics if the number is not a known IRQ.
///
/// # Safety
/// Must only be entered by hardware through an IRQ vector entry (exception number
/// 16 or above) on core 0. Never call it directly from software.
#[unsafe(no_mangle)]
unsafe extern "C" fn dispatch() {
    let ipsr = crate::register::ipsr::read();

    // IRQ n is exception number 16 + n.
    let Some(irq) = ipsr
        .checked_sub(16)
        .and_then(|n| u8::try_from(n).ok())
        .and_then(|n| Irq::try_from(n).ok())
    else {
        panic!("dispatch: invoked outside an IRQ vector entry");
    };

    unsafe { IRQ_HANDLER_TABLE.execute(irq) };
}
