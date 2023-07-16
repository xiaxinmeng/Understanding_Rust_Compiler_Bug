 rust
#![feature(unboxed_closures)]

fn main() {
    let i = 0i;
    let some_closure = |&: | {
        let _capture = i;
    };
}
