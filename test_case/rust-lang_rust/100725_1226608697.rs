rust
  --> /tmp/tto6.rs:15:9
   |
8  | impl<'a> Bigger<'a> {
   |      -- lifetime `'a` defined here
9  |     pub fn get_addr(byte_list: &'a mut Vec<u8>) -> &mut u8 {
10 |         byte_list.iter_mut().find_map(|item| {
   |         -------------------- first mutable borrow occurs here
11 |             Self::other(item); // replace with `Bigger`
   |             ----------- type annotation requires that `*byte_list` is borrowed for `'a`
...
15 |         byte_list.push(0);
   |         ^^^^^^^^^^^^^^^^^ second mutable borrow occurs here
   |
help: you can remove unnecessary lifetime annotations here
  --> /tmp/tto6.rs:11:13
   |
11 |             Self::other(item); // replace with `Bigger`
   |             ^^^^^^^^^^^
   = help: consider replacing `Self` with `Bigger`
