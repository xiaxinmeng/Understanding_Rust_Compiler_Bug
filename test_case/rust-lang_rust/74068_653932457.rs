
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable
 --> src/lib.rs:7:5
  |
3 | fn f1(map: &mut HashMap<i32, String>, k: i32) -> &String {
  |            - let's call the lifetime of this reference `'1`
4 |     if let Some(s) = map.get(&k) {
  |                      --- immutable borrow occurs here
5 |         return s;
  |                - returning this value requires that `*map` is borrowed for `'1`
6 |     }
7 |     map.insert(k, k.to_string());
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
