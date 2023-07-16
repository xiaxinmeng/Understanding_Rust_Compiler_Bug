rust
// #![feature(const_generics)]

trait F {
    type Item;
    fn next() -> Self::Item;
}

struct Foo<const D: usize>;
impl<const D: usize> F for Foo<{D}>
{
    type Item = [u8; D];
    fn next() -> Self::Item {
        return [0; D];
        return [0; D];
    }
}
