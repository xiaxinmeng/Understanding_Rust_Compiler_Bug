 rust
struct GenericStruct<T>;

impl <T> GenericStruct<T> {
    fn bar(&self, _: &mut T) {}
}

struct FooBar;

fn main() {
    let mut foobar = FooBar;
    let foo = GenericStruct;
    foo.bar(&mut foobar);
    foo.bar(&mut foobar);
}
