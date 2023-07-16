
#[derive(PartialEq)]
struct Foo<T> {
    bar: Bar<T>,
}

struct Bar<T> {}
