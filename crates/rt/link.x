MEMORY {
    FLASH : ORIGIN = 0x10000000, LENGTH = 2048K
    /* ram bank 0~7 */
    RAM : ORIGIN = 0x20000000, LENGTH = 512K
    /* ram bank 8 */
    SRAM4 : ORIGIN = 0x20080000, LENGTH = 4K
    /* ram bank 9 */
    SRAM5 : ORIGIN = 0x20081000, LENGTH = 4K
}

ENTRY(Reset);

EXTERN(RESET_VECTOR);
EXTERN(EXCEPTIONS);
EXTERN(IRQS)
EXTERN(IMAGE_DEF);

SECTIONS {
  .vector_table ORIGIN(FLASH) : 
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    KEEP(*(.vector_table.reset_vector));
    KEEP(*(.vector_table.exceptions));
    KEEP(*(.vector_table.irqs))
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
     . = ALIGN(4);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);

  /DISCARD/ : 
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}

SECTIONS {
  .image_def : ALIGN(4) {
    __image_def_addr = .;
    KEEP(*(.image_def));
  } > FLASH
} INSERT AFTER .vector_table;


_stext = ADDR(.image_def) + SIZEOF(.image_def);

PROVIDE(start_to_end = __end_block_addr - __start_block_addr);
PROVIDE(end_to_start = __start_block_addr - __end_block_addr);
PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SecureFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(DebugMonitor = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);
