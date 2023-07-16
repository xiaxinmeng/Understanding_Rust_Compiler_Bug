
fn mangle(
  &mut self,
  k: K,
  not_found: &fn(&K) -> V,
  found: &fn(&K, &mut V)) -> bool
