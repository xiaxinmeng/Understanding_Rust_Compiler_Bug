rust
trait StreamingIterator {
    type Item<'a>;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

fn foo<T: for<'a> StreamingIterator<Item<'a> = &'a [i32]>>(iter: T) { unimplemented!() }
