 rust
trait HasItem {
    type Item;
}

trait TDIterator<Item> {
    fn next(&mut self) -> Option<Item>;
}

trait BUIterator: HasItem + TDIterator<<Self as HasItem>::Item> {
}
