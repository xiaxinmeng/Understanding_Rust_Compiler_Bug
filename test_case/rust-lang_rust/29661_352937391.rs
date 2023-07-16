
trait Foo {}
trait Bar {}

trait Storage<T: Foo, U: Foo + Bar> {
    type Item = U;
    /// Get a value from the current key or None if it does not exist.
    fn get(&self, id: &T) -> Option<U>;
    /// Insert a value under the given key. Will overwrite the existing value.
    fn insert(&self, id: &T, item: &U);
    fn iter(&self) -> Iterator;
}

fn main () {
}
