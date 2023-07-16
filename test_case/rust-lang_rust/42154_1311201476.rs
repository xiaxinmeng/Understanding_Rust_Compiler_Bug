
error: reached the recursion limit while instantiating `fold::<&mut &mut &mut &mut &mut ...osure@src/main.rs:14:19: 14:25]>`
 --> src/main.rs:5:20
  |
5 |             init = fold(false, init, &mut f)
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: `fold` defined here
 --> src/main.rs:1:1
  |
1 | fn fold<F:FnMut(usize, u8) -> usize>(recurse: bool, mut init: usize, mut f: F) -> usize {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: the full type name has been written to '/playground/target/debug/deps/playground-07b5fd22867752b9.long-type.txt'
