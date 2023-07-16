
#![feature(macro_rules)]

macro_rules! foo(
    () => {
        unsafe { use std::mem; let () = mem::transmute(()); }
    }
)
fn main() {
    unsafe { foo!(); }
    foo!();
}
