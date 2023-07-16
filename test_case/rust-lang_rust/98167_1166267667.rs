rust
#![no_std]
#![no_main]
#![feature(bench_black_box)]

use core::hint::black_box;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    if let Some(value) = next() {
        print(value);
    }

    loop {
        //
    }
}

#[inline(never)]
fn next() -> Option<u16> { // this corresponds to `array.into_iter().next()` from the original code
                           // (note that the author doesn't call those functions by hand, but that's
                           //  how the loop gets desugared)
    black_box(Some(12345))
}

#[inline(never)]
fn print(n: u16) {
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let _ = ufmt::uwriteln!(serial, "{}", n);
}
