rust
#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::prelude::*;
use ufmt::uwriteln;

const BIG_NUMBER: u32 = 16_000;

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

    for x in (5u16..100u16).step_by(5) {
        for y in 1u16..5u16 {
            let z: u32 = BIG_NUMBER / x as u32 / y as u32;
            uwriteln!(&mut serial, "{} / {} = {:?}\r", x, y, z.to_le_bytes()).void_unwrap();
        }
    }

    loop {}
}
