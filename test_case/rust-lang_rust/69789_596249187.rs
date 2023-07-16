
error[E0596]: cannot borrow `*items` as mutable, as it is behind a `&` reference
  --> src/main.rs:10:9
   |
9  |     for items in map.values() {
   |         ----- help: consider changing this to be a mutable reference: `&mut std::vec::Vec<u8>`
10 |         items.sort();
   |         ^^^^^ `items` is a `&` reference, so the data it refers to cannot be borrowed as mutable
