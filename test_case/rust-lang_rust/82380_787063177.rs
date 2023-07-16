rust
#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::prelude::*;
use ufmt::uwriteln;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );

    for i in 0u16..16u16 {
        uwriteln!(&mut serial, "1 << {}\r", i).void_unwrap();

        let shl: u16 = 1u16 << i;
        uwriteln!(&mut serial, "Shift: {:?}\r", shl.to_le_bytes()).void_unwrap();

        let mul: u16 = 1u16 * 2u16.pow(i as u32);
        uwriteln!(&mut serial, "Mul and pow: {:?}\r\n", mul.to_le_bytes()).void_unwrap();
    }

    loop {}
}
