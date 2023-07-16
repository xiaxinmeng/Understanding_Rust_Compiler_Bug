
fn mangle<A>(
  &mut self,
  k: K,
  a: A,
  not_found: &fn(&K, A) -> V,
  found: &fn(&K, &mut V, A)) -> bool
