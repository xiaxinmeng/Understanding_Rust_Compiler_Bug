
error[E0597]: reference to `data` would be used after `data` is freed
 --> src/main.rs:4:5
   |
 3 | fn iterate(data: Rc<Vec<u32>>) -> impl Iterator<Item = u32> {
   |                                   -------------------------
   |                                   |
   |                                   as written, this type does not permit borrowed data
   |                                   help: try `impl Iterator<Item = u32> + '_`
 4 |     data.iter().cloned()
   |     ^^^^ `data` is borrowed here
 5 | }
   | - borrowed value only lives until here
   |
   = note: borrowed value must be valid for the static lifetime...
