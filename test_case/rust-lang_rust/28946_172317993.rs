
struct Foo<I> { parent: I,  }

impl<I: Iterator> Iterator for Foo<I> {
    type Item = Self::Item;

    fn next(&mut self) -> Option<I::Item> { None }
}
