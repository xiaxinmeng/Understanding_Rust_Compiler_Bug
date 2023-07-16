
Compiling playground v0.0.1 (/playground)
error: reached the recursion limit while instantiating `write_wrapped::<&mut &mut &mut &...&mut &mut &mut &mut &mut String>`
 --> src/main.rs:6:9
  |
6 |         write_wrapped(&mut w, s, false)
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: `write_wrapped` defined here
 --> src/main.rs:1:1
  |
1 | / fn write_wrapped<W>(mut w: W, s: &str, recurse: bool) -> std::fmt::Result
2 | | where
3 | |     W: std::fmt::Write,
  | |_______________________^
  = note: the full type name has been written to '/playground/target/debug/deps/playground-dd2e0560d12456cf.long-type.txt'

error: could not compile `playground` due to previous error
