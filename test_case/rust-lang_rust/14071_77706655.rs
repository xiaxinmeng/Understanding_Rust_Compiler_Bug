 rust
trait Foo {
    fn foo(self);
}

impl<'a> Foo for &'a mut u32 {
    fn foo(self) { }
}

fn main() {
    let mut x = 0;
    let y = &mut x;
    y.foo();
    y.foo();
}
