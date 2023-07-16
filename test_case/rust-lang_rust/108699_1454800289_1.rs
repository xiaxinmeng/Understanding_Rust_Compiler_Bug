text
error: `Struct::Trait` is and an associated constant
  --> $DIR/issue-108653-3.rs:11:7
   |
LL | /// [`Struct::Trait`]
   |       ^^^^^^^^^^^^^ ambiguous link
   |
note: the lint level is defined here
  --> $DIR/issue-108653-3.rs:4:9
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to link to the associated constant, prefix with `const@`
   |
LL | /// [`const@Struct::Trait`]
   |       ++++++

error: aborting due to previous error

