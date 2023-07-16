rust
fn main() {
    let baz = vec![0u8];
    Foo.bar(&baz); // Causes error
    Foo.bar(&*baz);
}

struct Foo;

trait Bar<T> {
    fn bar(&self, _: T) {}
}

impl Bar<&[u8]> for Foo {}
impl Bar<()> for Foo {} // Commenting it fixes error
