 rust
struct Foo<T>(T);

impl<T> Foo<T> {
    fn foo<U>(theta: U) -> Foo<T> {
        Bar::bar(theta)
    }
}

impl<T> Bar<T> for Foo<T> {
    fn bar<U>(_theta: U) -> Foo<T> { fail!(~"") }
}

trait Bar<T> {
    fn bar<U>(theta: U) -> Self;
}

fn main() {}
