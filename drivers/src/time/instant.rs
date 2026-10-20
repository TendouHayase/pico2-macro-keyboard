use crate::constant::{GCD_1G, GCD_1K, GCD_1M, TICK_HZ};

pub struct Instant {
    ticks: u64,
}

static mut GLOBAL_TICK: Instant = Instant { ticks: 0 };

impl Instant {
    pub const MIN: Instant = Instant { ticks: u64::MIN };
    pub const MAX: Instant = Instant { ticks: u64::MAX };

    #[inline]
    pub const fn now() -> Instant {
        Instant {
            ticks: unsafe { GLOBAL_TICK.ticks },
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
