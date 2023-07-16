rust
trait IntoIterator {
    type Item;
    type Iter: Iterator = impl Iterator<Item = <Self as IntoIterator>::Item>;
}
