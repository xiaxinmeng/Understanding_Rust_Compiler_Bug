
struct Foo(isize);

fn main() {
    let mut a: Foo;
    let _ = a.0 + 1;
}
