use embedded_hal::delay::DelayNs;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Delay;

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {}
}
