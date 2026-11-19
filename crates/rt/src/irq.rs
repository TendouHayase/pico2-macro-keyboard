type Irq = unsafe extern "C" fn();

unsafe extern "C" {
    fn Dispatch();
}

#[unsafe(link_section = ".vector_table.irqs")]
#[unsafe(no_mangle)]
pub static IRQS: [Irq; 52] = [Dispatch; 52];
