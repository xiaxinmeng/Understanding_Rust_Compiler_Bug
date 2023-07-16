
warning: `str` is both a builtin type and a macro
  --> src/lib.rs:47:14
   |
47 |         /// [str]
   |              ^^^ ambiguous link
   |
   = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default
help: to link to the builtin type, prefix with `prim@`
   |
47 |         /// [prim@str]
   |              +++++
help: to link to the macro, add an exclamation mark
   |
47 |         /// [str!]
   |                 +
