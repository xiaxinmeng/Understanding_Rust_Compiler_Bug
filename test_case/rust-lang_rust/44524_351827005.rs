rust
// These two give the same error that the iterator must be valid for the 'static lifetime:
fn foo<T>(items: &[T]) -> Box<Iterator<Item=&T>> {
    Box::new(items.iter())
}
fn foo<T>(items: &[T]) -> Box<Iterator<Item=&T> + '_> {
    Box::new(items.iter())
}

// But this one works:
fn foo<'a, T>(items: &'a [T]) -> Box<Iterator<Item=&'a T> + 'a> {
    Box::new(items.iter())
}
