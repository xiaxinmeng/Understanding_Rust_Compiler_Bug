 rust
struct Foo;

impl Drop for Foo { fn finalize(&self) {} }

fn main() {
    let y = Foo;
    let x = [y, ..10];
}
