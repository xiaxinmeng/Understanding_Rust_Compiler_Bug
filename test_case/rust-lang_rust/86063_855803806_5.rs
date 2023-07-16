rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::{dbg, debug};

#[entry]
fn main() -> ! {
    let x = dbg!(1i128);
    let x = unsafe { core::ptr::read_volatile(&x) };
    dbg!(x * x);
    debug::exit(debug::EXIT_SUCCESS);
    unreachable!();
}

#[panic_handler]
fn panic_handler(payload: &core::panic::PanicInfo) -> ! {
    dbg!(payload);
    debug::exit(debug::EXIT_FAILURE);
    unreachable!();
}
