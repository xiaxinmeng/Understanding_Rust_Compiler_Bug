 rust
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        panic!()
    }
}

fn abort() -> ! {
    let _x = Foo;
    panic!()
}

fn main() {
    abort();
}
