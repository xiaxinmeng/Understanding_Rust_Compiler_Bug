
fn insert_with(&mut self, key: K, value: V, f: &fn(&V, &V) -> V) -> Option<V>
fn insert_with_key(&mut self, key: K, value: V, f: &fn(&K, &V, &V) -> V) -> Option<V>
