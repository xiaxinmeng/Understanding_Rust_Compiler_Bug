rust
#![feature(never_type)]
#![allow(unreachable_code)]

fn main() {
    let y = &5;
    let x: ! = unsafe {
        *(y as *const _ as *const !) //~ ERROR entered unreachable code
    };
    f(x)
}

#[inline(never)]
fn f(x: !) -> ! { x }
