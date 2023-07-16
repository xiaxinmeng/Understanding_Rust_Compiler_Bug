
warning: lifetime name `'rots` only used once
  --> src/main.rs:20:62
   |
20 | fn rotations(n: u32, digs: &mut [u32], rots: &mut [u32]) -> &'rots [u32] {
   |                                                              ^^^^^
   |
note: lint level defined here
  --> src/main.rs:3:9
   |
3  | #![warn(single_use_lifetime)]
   |         ^^^^^^^^^^^^^^^^^^^

error[E0621]: explicit lifetime required in the type of `rots`
  --> src/main.rs:32:5
   |
20 | fn rotations(n: u32, digs: &mut [u32], rots: &mut [u32]) -> &'rots [u32] {
   |                                        ---- consider changing the type of `rots` to `&'rots mut [u32]`
...
32 |     &rots[.. pos]
   |     ^^^^^^^^^^^^^ lifetime `'rots` required
