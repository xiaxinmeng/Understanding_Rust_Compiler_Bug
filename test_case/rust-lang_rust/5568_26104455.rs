
fn insert_or_update_with<'a>(&'a mut self, k: K, v: V,
                             f: &fn(&K, &mut V)) -> &'a mut V {
    let cell = Cell::new(v);
    let (k, v) = self.find_or_insert_with(k, |_| cell.take());
    if !cell.is_empty() {
        f(k, v);
    }
    v
}
