 rust
pub fn find_or_insert<'a>(&'a mut self, k: K, v: V) -> &'a mut V {
    self.mangle(k, v, |_, v| v, |_, _, _| ())
}

pub fn find_or_insert_with<'a>(&'a mut self, k: K, f: |&K| -> V) -> &'a mut V {
    self.mangle(k, (), |k, _| f(k), |_, _, _| ())
}

pub fn insert_or_update_with<'a>(&'a mut self, k: K, v: V, f: |&K, &mut V|) -> &'a mut V {
    self.mangle(k, v, |_, v| v, |k, v, _| f(k, v))
}
