text
error: `u32::MAX` is both a trait and a builtin type
  --> $DIR/issue-108653-3.rs:4:7
   |
LL | /// [`u32::MAX`]
   |       ^^^^^^^^ ambiguous link
   |
note: the lint level is defined here
  --> $DIR/issue-108653-3.rs:1:9
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to link to the trait, prefix with `trait@`
   |
LL | /// [`trait@u32::MAX`]
   |       ++++++
help: to link to the builtin type, prefix with `prim@`
   |
LL | /// [`prim@u32::MAX`]
   |       +++++

error: aborting due to previous error
