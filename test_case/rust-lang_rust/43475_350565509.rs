rust
trait Foo {
    type Item;
    type FnOfItem: Fn(Self::Item);
}

fn test<T>()
where
    T: Foo<Item = usize>,
    //T::FnOfItem: Fn(T::Item), // <-- Compile error without this
{}

fn main() {}
