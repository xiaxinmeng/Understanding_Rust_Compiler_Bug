
warning: unresolved link to `Clone`
 --> ice.rs:1:6
  |
1 | //! [Clone()].
  |      ^^^^^^^ this link resolves to the trait `Clone`, which is not in the value namespace
  |
  = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default
help: to link to the trait, prefix with `trait@`
  |
1 - //! [Clone()].
1 + //! [trait@Clone].
  |

warning: 1 warning emitted
