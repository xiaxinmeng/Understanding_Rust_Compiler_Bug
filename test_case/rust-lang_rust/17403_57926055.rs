 rust
#![feature(unboxed_closures, overloaded_calls)]

fn main() {
    let mut x = 0u;
    let mut f = |&mut:| &mut x;
    let _y = f();
    let _z = f();
}
