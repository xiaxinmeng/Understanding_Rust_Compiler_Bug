 rs
// self[element] in an rvalue context
trait IndexGet<E,R> {
    fn index_get(&self, element: &E) -> R;
}
