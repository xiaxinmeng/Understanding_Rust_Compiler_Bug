rust
#![no_std]
#![no_main]

use core::sync::atomic::{Ordering, AtomicUsize};

#[panic_handler]
fn handle_panic(_arg: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let x = AtomicUsize::new(0);
    x.load(Ordering::SeqCst);
    loop { }
}
