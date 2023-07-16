 rust
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println("dropping");
    }
}

fn main() {
    let x = true;
    if x {
        let a = Foo;
    }
}
