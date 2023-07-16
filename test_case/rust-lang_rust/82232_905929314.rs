rust
#![feature(fn_align)]

#[repr(align(1))]
fn main() {
    Bar::foo();
}

struct Bar;

impl Bar {
    #[repr(align(1))]
    fn foo() {
        println!("bar");
    }
}
