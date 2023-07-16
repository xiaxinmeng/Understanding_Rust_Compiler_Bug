rust
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_0() {
    let mut item = 0u8;
    use_var(&mut item);
}

#[inline(never)]
pub fn use_var(item: &mut u8) { *item = 0; }

#[export_name = "main"]
pub extern "C" fn main() -> ! { loop {} }

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
