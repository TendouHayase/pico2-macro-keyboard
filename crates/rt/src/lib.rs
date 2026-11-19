#![no_std]

use core::{arch::global_asm, panic::PanicInfo};

mod boot_info;
mod irq;
mod vector;

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[unsafe(export_name = "main")]
        pub unsafe extern "C" fn __main() -> ! {
            let f: fn() -> ! = $path;

            f();
        }
    };
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

global_asm!(include_str!("boot.s"));

unsafe extern "C" {
    fn Reset();
}

#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static RESET_VECTOR: unsafe extern "C" fn() = Reset;
