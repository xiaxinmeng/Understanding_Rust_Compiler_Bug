rust
pub trait Foo<T> {
    type Iter<'a>: Iterator<Item=&'a T>;
}
