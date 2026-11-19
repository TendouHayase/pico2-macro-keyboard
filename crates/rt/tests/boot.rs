//! rt 부팅 검증 테스트: `cargo test -p rp2350-rt --test boot`
//!
//! - `.bss`가 0으로 초기화되는가
//! - `.data`가 플래시의 초기값으로 복사되는가
//! - `.rodata`를 읽을 수 있는가
//! - FPU가 켜져 있는가 (꺼져 있으면 연산에서 UsageFault → HardFault)
//!
//! 디버거가 붙어 있지 않은 상태에서 실행하면 `bkpt`가 HardFault가 되니,
//! 반드시 `cargo test`(= probe-rs run)로 실행

#![no_std]
#![no_main]

use core::hint::black_box;
use core::ptr::read_volatile;

use rp2350_rt::entry;

/// `.bss`: 초기값 0
#[unsafe(no_mangle)]
static mut TEST_BSS: u8 = 0;

/// `.data`: 0이 아닌 초기값
#[unsafe(no_mangle)]
static mut TEST_DATA: u8 = 0xA5;

/// `.rodata`: `_sidata`(=.data의 로드 주소) 위치에 영향을 줌
#[unsafe(no_mangle)]
static TEST_RODATA: u8 = 0x5A;

entry!(main);

fn main() -> ! {
    let bss = unsafe { read_volatile(&raw const TEST_BSS) };
    let data = unsafe { read_volatile(&raw const TEST_DATA) };
    let rodata = unsafe { read_volatile(&raw const TEST_RODATA) };

    let a = black_box(1.5f32);
    let b = black_box(2.25f32);
    let fpu = black_box(a * b + a);

    let ok = bss == 0 && data == 0xA5 && rodata == 0x5A && fpu == 4.875;

    exit(ok)
}

/// Arm semihosting SYS_EXIT (0x18)로 디버거에 종료를 알립니다.
/// r1: 0x20026 = ADP_Stopped_ApplicationExit (성공),
///     0x20023 = ADP_Stopped_RunTimeErrorUnknown (실패)
fn exit(success: bool) -> ! {
    let reason: u32 = if success { 0x2_0026 } else { 0x2_0023 };
    unsafe {
        core::arch::asm!(
            "bkpt 0xAB",
            in("r0") 0x18u32,
            in("r1") reason,
            options(nostack),
        );
    }
    loop {}
}
