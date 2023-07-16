
use std::collections::HashMap;

pub fn main() {
  let mut xs = HashMap::<(u32, u32), u32>::new();
  let new_el = |v| v.insert(30);  // error: the type of this value must be known in this context

  xs.insert((1,1), 10);
  xs.insert((2,2), 20);
  xs.entry((3,3)).get().unwrap_or_else(new_el);

  println!("{}", xs);
}
