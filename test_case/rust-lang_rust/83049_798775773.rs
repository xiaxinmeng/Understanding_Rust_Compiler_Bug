
warning: unresolved link to `self::bar::baz`
 --> foo.rs:2:18
  |
2 |     //! Look at [crate::bar::baz].
  |                  ^^^^^^^^^^^^^^^ no item named `baz` in module `bar`
  |
  = note: `#[warn(broken_intra_doc_links)]` on by default

warning: 1 warning emitted
