
use std::mem;

fn main() {
    let x = 5;

    let y = &x;

    let z = unsafe { mem::transmute::<&i32, &mut i32>(y) };
}
