#![no_std]
#![no_main]

use rp2350_drivers as _;
use rp2350_rt::entry;

entry!(main);

fn main() -> ! {
    loop {}
}
