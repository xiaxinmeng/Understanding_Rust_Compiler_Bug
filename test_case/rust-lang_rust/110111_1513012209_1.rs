
warning: incompatible link kind for `Clone`
   --> my_crate/src/lib.rs:175:23
    |
175 | /// For bug report: [`struct@Clone`]
    |                       ^^^^^^^^^^^^ this link resolved to a trait, which is not a struct
    |
    = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default
help: to link to the trait, prefix with `trait@`
    |
175 | /// For bug report: [`trait@lone`]
    |                       ~~~~~~
