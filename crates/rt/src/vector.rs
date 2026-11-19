pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

unsafe extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SecureFault();
    fn SVCall();
    fn DebugMonitor();
    fn PendSV();
    fn SysTick();
}

#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector {
        handler: SecureFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector {
        handler: DebugMonitor,
    },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[unsafe(no_mangle)]
pub extern "C" fn DefaultExceptionHandler() {
    panic!();
}
