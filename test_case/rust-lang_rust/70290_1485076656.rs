
error[[E0283]](https://doc.rust-lang.org/nightly/error_codes/E0283.html): type annotations needed: cannot satisfy `F: FnOnce<(&'static i32,)>`
 --> src/main.rs:4:8
  |
4 |     F: FnOnce(&'static i32) -> &'static i32
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: multiple `impl`s or `where` clauses satisfying `F: FnOnce<(&'static i32,)>` found
 --> src/main.rs:3:8
  |
3 |     F: for<'a> FnOnce(&'a i32) -> &'a i32,
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
4 |     F: FnOnce(&'static i32) -> &'static i32
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0283`.
