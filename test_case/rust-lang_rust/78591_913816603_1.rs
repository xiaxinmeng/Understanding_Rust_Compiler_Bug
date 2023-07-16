
warning: unresolved link to `Foo`
 --> src/lib.rs:5:11
  |
5 |     //! [`Foo`]
  |           ^^^ no item named `Foo` in scope
  |
note: the lint level is defined here
 --> src/lib.rs:1:8
  |
1 | #[warn(rustdoc::broken_intra_doc_links)]
  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
