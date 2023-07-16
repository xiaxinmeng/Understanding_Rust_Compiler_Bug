 rs
// self[element] -- if used as rvalue, implicitly a deref of the result
trait Index<E,R> {
    fn index<'a>(&'a self, element: &E) -> &'a R;
}

// &mut self[element] -- when used as a mutable lvalue
trait IndexMut<E,R> {
    fn index_mut<'a>(&'a mut self, element: &E) -> &'a mut R;
}

// self[element] = value
trait IndexSet<E,V> {
    fn index_set(&mut self, element: E, value: V);
}
