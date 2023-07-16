Rust
#![feature(let_else)]

#[allow(unused)]
enum Droppy {
    A(u32),
    B,
}

impl Drop for Droppy {
    fn drop(&mut self) {
        println!("I dropped");
    }
}

fn foo() {
    let &Droppy::A(_v) = &Droppy::B else {
        println!("Should have dropped");
        return;
    };
}

fn main() {
    foo();
}
