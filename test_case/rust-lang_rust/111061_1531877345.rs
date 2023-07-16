plain
80 |     Initial,
81 |     Final,
   |     ^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
error: variant `Final` is never constructed
  --> compiler/rustc_mir_transform/src/gvn.rs:49:5
   |
   |
47 | pub enum GVN {
   |          --- variant in this enum
49 |     Final,
   |     ^^^^^

error: could not compile `rustc_mir_transform` (lib) due to 2 previous errors
