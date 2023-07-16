rust
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let pixels = [0];

    for pixel in pixels.map(|n| n + 1) { // works with `.into_iter().map()`
        print(pixel);
    }

    loop {
        //
    }
}

#[inline(never)]
fn print(pixel: u16) {
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let _ = ufmt::uwriteln!(serial, "{}", pixel);
}
