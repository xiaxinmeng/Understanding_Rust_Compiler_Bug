rust
#[repr(align(1073741824))]
struct Foo;

fn main() {
    Box::new(Foo);
}
