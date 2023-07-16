text
error: `Trait` is both a trait and a constant
  --> $DIR/issue-108653-3.rs:12:7
   |
LL | /// [`Trait`]
   |       ^^^^^ ambiguous link
   |
note: the lint level is defined here
  --> $DIR/issue-108653-3.rs:4:9
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to link to the trait, prefix with `trait@`
   |
LL | /// [`trait@Trait`]
   |       ++++++
help: to link to the constant, prefix with `const@`
   |
LL | /// [`const@Trait`]
   |       ++++++

error: `Trait::Trait` is both a trait and a trait
  --> $DIR/issue-108653-3.rs:14:7
   |
LL | /// [`Trait::Trait`]
   |       ^^^^^^^^^^^^ ambiguous link
   |
help: to link to the trait, prefix with `trait@`
   |
LL | /// [`trait@Trait::Trait`]
   |       ++++++
help: to link to the trait, prefix with `trait@`
   |
LL | /// [`trait@Trait::Trait`]
   |       ++++++

error: aborting due to 2 previous errors
