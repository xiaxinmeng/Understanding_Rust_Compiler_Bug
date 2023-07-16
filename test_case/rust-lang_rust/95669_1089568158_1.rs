
error: missing fragment specifier
 --> v.rs:2:25
  |
2 | macro_rules! m_used { ( $id ) => {} }
  |                         ^^^

error: missing fragment specifier
 --> v.rs:3:27
  |
3 | macro_rules! m_unused { ( $id ) => {} }
  |                           ^^^
  |
  = note: `#[deny(missing_fragment_specifier)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>
