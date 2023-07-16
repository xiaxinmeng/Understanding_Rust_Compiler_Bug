 rust
#![feature(unboxed_closures)]

fn main() {
    let i: isize = 0;
    let some_closure = || {
        let _capture = i;
    };
}
