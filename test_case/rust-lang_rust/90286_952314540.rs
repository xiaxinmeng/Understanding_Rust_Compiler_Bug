
error[E0507]: cannot move out of `*v` which is behind a shared reference
  --> src/lib.rs:26:13
   |
26 |       let r = v.long_function_with_many_params(
   |  _____________^
27 | |         &[1, 2, 3, 4],
28 | |         12..42,
29 | |         b"ababababbaabab",
...  |
32 | |         Flag::Yes,
33 | |     )?;
   | |_____^ move occurs because `*v` has type `Type`, which does not implement the `Copy` trait
