 rust
use std::util::with;
use std::hashmap::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 1);

  macro_rules! add_foo(
    ($a1:expr, $b1:expr) => ({
      map.find(&$a1).map(|&m| *m + $b1);
    })
  );

  macro_rules! foo(
    ($a:expr, $b:expr) => ({
      add_foo!($a, $b);
      add_foo!($b, $a);
    })
  );

  let def = 1;
  foo!(def, def);
}
