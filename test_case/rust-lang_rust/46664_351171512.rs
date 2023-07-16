Rust
trait Foo {
    fn bar(self: *const Self);
}

fn main() {
    let foo: *const Foo = mem::transmute([0usize, 0usize]);
    foo.bar(); // this is UB
}
