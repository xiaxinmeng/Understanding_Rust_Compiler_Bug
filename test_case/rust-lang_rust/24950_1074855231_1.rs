
Compiling playground v0.0.1 (/playground)
error[[E0499]](https://doc.rust-lang.org/stable/error-index.html#E0499): cannot borrow `*k` as mutable more than once at a time
  --> src/main.rs:14:13
   |
10 | impl<'a> LayoutDamageComputation for &'a mut (dyn Flow + 'a) {
   |      -- lifetime `'a` defined here
...
13 |             k.compute_layout_damage();
   |             -------------------------
   |             |
   |             first mutable borrow occurs here
   |             argument requires that `*k` is borrowed for `'a`
14 |             k.compute_layout_damage();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^ second mutable borrow occurs here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `playground` due to previous error
