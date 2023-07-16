rust
fn bar() {
    loop {}
}

pub trait Trait {
    fn foo(&self) {
     // if true {
            bar();
     // }
    }
}

impl Trait for u8 {}

fn main() {
    println!("hi")
}
