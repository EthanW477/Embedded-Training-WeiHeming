use embassy_stm32::{
    bind_interrupts,
    can::Can,
    gpio::{Flex, Level, Output, Speed},
    peripherals,
    usart::{self, BufferedUart, Config},
    Peripherals,
};
use static_cell::StaticCell;

use crate::swd::Swd;

bind_interrupts!(struct UartIrqs {
    UART8 => usart::BufferedInterruptHandler<peripherals::UART8>;
});

static UART_TX_BUFFER: StaticCell<[u8; 128]> = StaticCell::new();
static UART_RX_BUFFER: StaticCell<[u8; 128]> = StaticCell::new();

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // PE09: SWCLK output; PE13: bidirectional SWDIO.
    let swd = Some(Swd::new(
        Output::new(p.PE9, Level::Low, Speed::Low),
        Flex::new(p.PE13),
    ));

    // UART8: PE00 RX, PE01 TX; default framing is 8N1.
    let mut uart_config = Config::default();
    uart_config.baudrate = 115_200;

    let uart = Some(
        BufferedUart::new(
            p.UART8,
            p.PE0,
            p.PE1,
            UART_TX_BUFFER.init([0; 128]),
            UART_RX_BUFFER.init([0; 128]),
            UartIrqs,
            uart_config,
        )
        .expect("UART8 configuration failed"),
    );

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);

    (usb, Ports { can, uart }, swd)
}