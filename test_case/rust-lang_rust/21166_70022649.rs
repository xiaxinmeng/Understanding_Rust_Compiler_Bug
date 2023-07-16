 rust
#[deriving(Copy)]
struct Foo;
fn main() {
    let x = Foo;
    // use it twice:
    (x, x);
}
