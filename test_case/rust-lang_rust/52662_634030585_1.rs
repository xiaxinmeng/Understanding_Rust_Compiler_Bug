rust
type Item = impl Debug;
fn foo() -> impl IntoIterator<Item = Item, IntoIter: Iterator<Item = Item> + Debug> {...}
