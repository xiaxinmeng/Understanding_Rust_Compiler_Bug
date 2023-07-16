 rust
#[unsafe_no_drop_flag]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() { }
