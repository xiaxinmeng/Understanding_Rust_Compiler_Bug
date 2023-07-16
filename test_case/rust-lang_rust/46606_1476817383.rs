rs
pub trait Foo {
    type Assoc;

    fn foo<F, B>(&self, _: F)
    where
        F: FnOnce() -> B,
        B: Foo<Assoc = Self::Assoc>,
    {}
}

impl<T> Foo for T {
    type Assoc = T;
}

fn item<T>(item: T) -> T {
    item
}

fn main() {
    item("&'static str error")
        .foo(|| "String error".to_owned());
}
