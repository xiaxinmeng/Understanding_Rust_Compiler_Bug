
warning: unresolved link to `Bar::bar`
 --> test.rs:3:28
  |
3 | /// You should really try [`Self::bar`]
  |                            ^^^^^^^^^^^ the type alias `Bar` has no associated item named `bar`
  |
  = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default

warning: 1 warning emitted
