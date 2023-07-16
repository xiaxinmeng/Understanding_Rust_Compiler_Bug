 rust
pub type MapFn<I, B> where I: Iterator = iter::Map<I, fn(I::Item) -> B>;
