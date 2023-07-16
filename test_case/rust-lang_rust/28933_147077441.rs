
#[derive(PartialEq)]
struct Foo<T> {
    f: T,
}

struct Bar;

fn main() {
    let x = Foo { f: Bar };
    x == x;
}
