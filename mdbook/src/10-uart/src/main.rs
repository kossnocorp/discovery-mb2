#![no_main]
#![no_std]

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    for char in "The quick brown fox jumps over the lazy dog."
        .as_bytes()
        .iter()
    {
        serial.write(*char).unwrap();
    }
    serial.flush().unwrap();

    loop {
        wfi();
    }
}
