use interface::{Error, ErrorKind};

pub const IRQ_COUNT: usize = 52;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Irq {
    /// `TIMER0_IRQ_0`
    Timer0Irq0 = 0,
    /// `TIMER0_IRQ_1`
    Timer0Irq1 = 1,
    /// `TIMER0_IRQ_2`
    Timer0Irq2 = 2,
    /// `TIMER0_IRQ_3`
    Timer0Irq3 = 3,
    /// `TIMER1_IRQ_0`
    Timer1Irq0 = 4,
    /// `TIMER1_IRQ_1`
    Timer1Irq1 = 5,
    /// `TIMER1_IRQ_2`
    Timer1Irq2 = 6,
    /// `TIMER1_IRQ_3`
    Timer1Irq3 = 7,
    /// `PWM_IRQ_WRAP_0`
    PwmIrqWrap0 = 8,
    /// `PWM_IRQ_WRAP_1`
    PwmIrqWrap1 = 9,
    /// `DMA_IRQ_0`
    DmaIrq0 = 10,
    /// `DMA_IRQ_1`
    DmaIrq1 = 11,
    /// `DMA_IRQ_2`
    DmaIrq2 = 12,
    /// `DMA_IRQ_3`
    DmaIrq3 = 13,
    /// `USBCTRL_IRQ`
    UsbctrlIrq = 14,
    /// `PIO0_IRQ_0`
    Pio0Irq0 = 15,
    /// `PIO0_IRQ_1`
    Pio0Irq1 = 16,
    /// `PIO1_IRQ_0`
    Pio1Irq0 = 17,
    /// `PIO1_IRQ_1`
    Pio1Irq1 = 18,
    /// `PIO2_IRQ_0`
    Pio2Irq0 = 19,
    /// `PIO2_IRQ_1`
    Pio2Irq1 = 20,
    /// `IO_IRQ_BANK0`
    IoIrqBank0 = 21,
    /// `IO_IRQ_BANK0_NS`
    IoIrqBank0Ns = 22,
    /// `IO_IRQ_QSPI`
    IoIrqQspi = 23,
    /// `IO_IRQ_QSPI_NS`
    IoIrqQspiNs = 24,
    /// `SIO_IRQ_FIFO`
    SioIrqFifo = 25,
    /// `SIO_IRQ_BELL`
    SioIrqBell = 26,
    /// `SIO_IRQ_FIFO_NS`
    SioIrqFifoNs = 27,
    /// `SIO_IRQ_BELL_NS`
    SioIrqBellNs = 28,
    /// `SIO_IRQ_MTIMECMP`
    SioIrqMtimecmp = 29,
    /// `CLOCKS_IRQ`
    ClocksIrq = 30,
    /// `SPI0_IRQ`
    Spi0Irq = 31,
    /// `SPI1_IRQ`
    Spi1Irq = 32,
    /// `UART0_IRQ`
    Uart0Irq = 33,
    /// `UART1_IRQ`
    Uart1Irq = 34,
    /// `ADC_IRQ_FIFO`
    AdcIrqFifo = 35,
    /// `I2C0_IRQ`
    I2c0Irq = 36,
    /// `I2C1_IRQ`
    I2c1Irq = 37,
    /// `OTP_IRQ`
    OtpIrq = 38,
    /// `TRNG_IRQ`
    TrngIrq = 39,
    /// `PROC0_IRQ_CTI`
    Proc0IrqCti = 40,
    /// `PROC1_IRQ_CTI`
    Proc1IrqCti = 41,
    /// `PLL_SYS_IRQ`
    PllSysIrq = 42,
    /// `PLL_USB_IRQ`
    PllUsbIrq = 43,
    /// `POWMAN_IRQ_POW`
    PowmanIrqPow = 44,
    /// `POWMAN_IRQ_TIMER`
    PowmanIrqTimer = 45,
    /// `SPARE_IRQ_0`
    SpareIrq0 = 46,
    /// `SPARE_IRQ_1`
    SpareIrq1 = 47,
    /// `SPARE_IRQ_2`
    SpareIrq2 = 48,
    /// `SPARE_IRQ_3`
    SpareIrq3 = 49,
    /// `SPARE_IRQ_4`
    SpareIrq4 = 50,
    /// `SPARE_IRQ_5`
    SpareIrq5 = 51,
}

impl TryFrom<u8> for Irq {
    type Error = InvalidIrq;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Irq::Timer0Irq0),
            1 => Ok(Irq::Timer0Irq1),
            2 => Ok(Irq::Timer0Irq2),
            3 => Ok(Irq::Timer0Irq3),
            4 => Ok(Irq::Timer1Irq0),
            5 => Ok(Irq::Timer1Irq1),
            6 => Ok(Irq::Timer1Irq2),
            7 => Ok(Irq::Timer1Irq3),
            8 => Ok(Irq::PwmIrqWrap0),
            9 => Ok(Irq::PwmIrqWrap1),
            10 => Ok(Irq::DmaIrq0),
            11 => Ok(Irq::DmaIrq1),
            12 => Ok(Irq::DmaIrq2),
            13 => Ok(Irq::DmaIrq3),
            14 => Ok(Irq::UsbctrlIrq),
            15 => Ok(Irq::Pio0Irq0),
            16 => Ok(Irq::Pio0Irq1),
            17 => Ok(Irq::Pio1Irq0),
            18 => Ok(Irq::Pio1Irq1),
            19 => Ok(Irq::Pio2Irq0),
            20 => Ok(Irq::Pio2Irq1),
            21 => Ok(Irq::IoIrqBank0),
            22 => Ok(Irq::IoIrqBank0Ns),
            23 => Ok(Irq::IoIrqQspi),
            24 => Ok(Irq::IoIrqQspiNs),
            25 => Ok(Irq::SioIrqFifo),
            26 => Ok(Irq::SioIrqBell),
            27 => Ok(Irq::SioIrqFifoNs),
            28 => Ok(Irq::SioIrqBellNs),
            29 => Ok(Irq::SioIrqMtimecmp),
            30 => Ok(Irq::ClocksIrq),
            31 => Ok(Irq::Spi0Irq),
            32 => Ok(Irq::Spi1Irq),
            33 => Ok(Irq::Uart0Irq),
            34 => Ok(Irq::Uart1Irq),
            35 => Ok(Irq::AdcIrqFifo),
            36 => Ok(Irq::I2c0Irq),
            37 => Ok(Irq::I2c1Irq),
            38 => Ok(Irq::OtpIrq),
            39 => Ok(Irq::TrngIrq),
            40 => Ok(Irq::Proc0IrqCti),
            41 => Ok(Irq::Proc1IrqCti),
            42 => Ok(Irq::PllSysIrq),
            43 => Ok(Irq::PllUsbIrq),
            44 => Ok(Irq::PowmanIrqPow),
            45 => Ok(Irq::PowmanIrqTimer),
            46 => Ok(Irq::SpareIrq0),
            47 => Ok(Irq::SpareIrq1),
            48 => Ok(Irq::SpareIrq2),
            49 => Ok(Irq::SpareIrq3),
            50 => Ok(Irq::SpareIrq4),
            51 => Ok(Irq::SpareIrq5),
            invalid => Err(InvalidIrq(invalid)),
        }
    }
}

/// The value is not a valid RP2350 IRQ number.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InvalidIrq(pub u8);

impl Error for InvalidIrq {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}
