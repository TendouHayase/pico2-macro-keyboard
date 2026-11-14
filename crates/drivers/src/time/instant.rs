use crate::constant::{GCD_1G, GCD_1K, GCD_1M, TICK_HZ};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    ticks: u64,
}

impl Instant {
    pub const MIN: Instant = Instant { ticks: u64::MIN };
    pub const MAX: Instant = Instant { ticks: u64::MAX };

    #[inline]
    pub fn now() -> Instant {
        Instant {
            ticks: read_global_tick(),
        }
    }

    pub const fn from_ticks(ticks: u64) -> Self {
        Self { ticks }
    }

    pub const fn from_nanos(nanos: u64) -> Self {
        Self {
            ticks: nanos * (TICK_HZ / GCD_1G) / (1_000_000_000 / GCD_1G),
        }
    }

    pub const fn from_micros(micros: u64) -> Self {
        Self {
            ticks: micros * (TICK_HZ / GCD_1M) / (1_000_000 / GCD_1M),
        }
    }

    pub const fn from_millis(millis: u64) -> Self {
        Self {
            ticks: millis * (TICK_HZ / GCD_1K) / (1_000 / GCD_1K),
        }
    }

    pub const fn from_secs(secs: u64) -> Self {
        Self {
            ticks: secs * TICK_HZ,
        }
    }
}

static mut GLOBAL_TICK: Instant = Instant { ticks: 0 };

#[unsafe(no_mangle)]
extern "C" fn increase_global_tick() {
    // SAFETY: Called only from the timer ISR, so there is a single writer.
    // On a single core, thread-mode readers cannot preempt this ISR, and no
    // higher-priority ISR reads the tick, so no critical section is needed.
    // Place access only; no reference to `GLOBAL_TICK` is created.
    unsafe { GLOBAL_TICK.ticks += 1 };
}

fn read_global_tick() -> u64 {
    // SAFETY: The 64-bit load is not atomic on Cortex-M33; interrupts are
    // disabled inside `free`, so the timer ISR cannot cause a torn read.
    // Relies on `free` acting as a compiler barrier. Value is copied; no
    // reference to `GLOBAL_TICK` is created.
    crate::interrupt::free(|| unsafe { GLOBAL_TICK.ticks })
}
