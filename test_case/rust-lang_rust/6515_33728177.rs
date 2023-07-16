 rs
// self[element] = value
trait IndexSetRef<E,V> {
    fn index_set(&mut self, element: &E, value: V);
}
