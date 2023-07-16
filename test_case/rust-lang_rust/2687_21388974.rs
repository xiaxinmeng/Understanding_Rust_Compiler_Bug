 rust
trait Foo {
    unsafe fn x();
}

impl Foo for int {
    unsafe fn x() {}
}
impl Foo for uint {
    fn x() {}
}

fn main() {}
