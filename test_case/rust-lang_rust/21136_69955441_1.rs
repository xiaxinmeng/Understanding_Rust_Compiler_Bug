 rust
struct GenericStruct<T>;

impl <T> GenericStruct<T> {
    fn bar(&self, _: T) {}                   // <--- Only change
}

struct FooBar;

fn main() {
    let mut foobar = FooBar;
    let foo = GenericStruct;
    foo.bar(&mut foobar);
    foo.bar(&mut foobar);
}
