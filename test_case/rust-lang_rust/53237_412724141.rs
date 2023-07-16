rust
#![no_std]
#![feature(lang_items, panic_implementation, core_intrinsics)]

#[panic_implementation]
pub fn panic_fmt(_fmt: &core::panic::PanicInfo) -> ! {
    unsafe { core::intrinsics::abort() }
}

#[lang = "eh_personality"]
pub fn eh_personality() {}

extern {
    fn test(f: extern fn() -> i32);
}

#[lang = "start"]
pub fn main() {
    extern fn cb() -> i32 {
        123
    }

    unsafe {
        test(cb);
    }
}
