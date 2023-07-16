 rust
#![feature(std_misc)]
use std::collections::HashMap;
use std::collections::hash_map::VacantEntry;
pub fn main() {
  let mut xs = HashMap::<(u32, u32), u32>::new();
  let new_el = |v:VacantEntry<(u32, u32),u32>| v.insert(30);
  xs.insert((1,1), 10);
  xs.insert((2,2), 20);
  xs.entry((3,3)).get().unwrap_or_else(new_el);
  println!("{:?}", xs);
}
