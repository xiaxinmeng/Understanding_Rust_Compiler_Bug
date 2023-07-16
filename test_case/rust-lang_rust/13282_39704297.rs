 Rust
#![allow(dead_code, unreachable_code)]
#![crate_type = "lib"]
pub fn fall() -> ! {
    unsafe { std::cast::transmute(()) }
}

pub fn foo() {
    fall();
    // dead & unreachable from here
    match () {
        () => {
            static A: int = 3;
        }
    }
}
