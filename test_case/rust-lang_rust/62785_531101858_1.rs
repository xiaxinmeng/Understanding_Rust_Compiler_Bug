rust
#![no_std]
#![no_main]
use core::fmt::{Write,Error};

struct Dummy;

impl Write for Dummy {
    fn write_str(&mut self, _s: &str) -> Result<(), Error>{
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn efi_main() -> i32 {
    write!(Dummy,"Hello, world!").unwrap();
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}
