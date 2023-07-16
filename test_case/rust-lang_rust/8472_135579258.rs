 rust
macro_rules! foo {
    (unsafe $e:expr) => {
        unsafe { foo!($e) }
    };
    ($e:expr) => {{
       use std::mem;
       mem::transmute($e)
    }}
}
#[allow(unused_variables)]
fn main() {
    let x: u32 = unsafe { foo!(1) };
    let y: u32 = foo!(unsafe 1);
}
