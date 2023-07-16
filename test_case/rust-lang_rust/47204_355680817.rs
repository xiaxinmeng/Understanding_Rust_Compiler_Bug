rust
trait Foo {
    fn bar(&self) {}
}

impl Foo for unsafe fn() {}

unsafe fn baz() {}

fn main() {
    // These do not work:
    baz.bar();
    Foo::bar(&baz);

    // These do:
    let x: unsafe fn() = baz;
    x.bar();
    Foo::bar(&x);
}
