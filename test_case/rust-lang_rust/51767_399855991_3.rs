rust
#![feature(const_fn)]

const fn foo() -> i32 {
    1
}

fn main() {
    [(); { foo() } ];
}
