rust
pub trait Foo {
    type Bar: std::ops::Add<Self::Bar, Output = Self::Bar> + std::fmt::Debug;
    fn bar(self) -> Self::Bar;
}

impl<T: std::ops::Add<T, Output = T> + std::fmt::Debug> Foo for T {
    type Bar = T;
    fn bar(self) -> Self::Bar {
        self
    }
}

pub fn foo() -> impl Foo<Bar = impl std::ops::Sub<usize>> {
    5usize
}

fn baz<T: Foo>(foo1: T, foo2: T) { dbg!(foo1.bar() + foo2.bar()); }

fn main() {
    baz(foo(), foo());
}
