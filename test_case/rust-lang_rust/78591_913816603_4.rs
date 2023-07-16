
warning: unresolved link to `Foo`
 --> src/lib.rs:3:1
  |
2 | |
  | |___________^
3 | / /// We have some module docs here.
  |
note: the lint level is defined here
 --> src/lib.rs:1:8
  |
1 | #[warn(rustdoc::broken_intra_doc_links)]
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: the link appears in this line:

          [`Foo`]
           ^^^^^
  = note: no item named `Foo` in scope
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
