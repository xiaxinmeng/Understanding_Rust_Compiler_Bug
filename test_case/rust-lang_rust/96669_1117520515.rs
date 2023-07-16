rust
trait IteratorItem<'a> {
    type Item: 'static;
}

trait Iterator: for<'a> IteratorItem<'a> {
    fn next(&mut self) -> <Self as IteratorItem<'_>>::Item;
}

fn not_ok<T>(mut it: T) where T: Iterator {
    let mut _a = it.next();
    let mut _b = it.next();
}
