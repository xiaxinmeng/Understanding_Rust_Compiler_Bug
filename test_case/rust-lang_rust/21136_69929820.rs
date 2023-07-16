 Rust
struct GenericStruct<T: ?Sized>;

impl <T: ?Sized> GenericStruct<T> {
    fn bar(&self, _: &mut T) {}
}

trait Trait{}

struct FooBar;
impl Trait for FooBar{}

fn main() {
    let mut foobar = FooBar;
    let foo = GenericStruct::<Trait>;
    foo.bar(&mut foobar);   // First borrow passes ...
    foo.bar(&mut foobar);   // Second borrow
}
