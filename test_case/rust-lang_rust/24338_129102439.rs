 rust
trait DictLike<'a> {
    type ItemsIterator: Iterator<Item=u8>;
    fn get(mut c: Self::ItemsIterator) {
        c = c.into_iter();
    }
}
fn main() {}
