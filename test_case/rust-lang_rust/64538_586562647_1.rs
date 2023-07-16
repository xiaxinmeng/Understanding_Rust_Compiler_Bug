
error: reached the recursion limit while instantiating `recurse::<&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&[closure@src/main.rs:7:13: 7:18]>`
 --> src/main.rs:2:1
  |
2 | / fn recurse(f: impl Fn()) {
3 | |     recurse(&f)
4 | | }
  | |_^
