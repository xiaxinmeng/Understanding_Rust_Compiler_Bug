 rust
#![crate_type = "lib"]
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
