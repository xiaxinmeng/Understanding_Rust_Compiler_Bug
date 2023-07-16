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

    for x in (5i32..10i32).step_by(5) {
        for y in 1i32..5i32 {
            let z = x / y;
            uwriteln!(&mut serial, "{} / {} = {:?}\r", x, y, z.to_le_bytes()).void_unwrap();
        }
    }

    loop {}
}
