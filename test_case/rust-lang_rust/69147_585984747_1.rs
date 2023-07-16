rust

error[E0404]: expected trait, found builtin type `u8`
 --> src/lib.rs:1:13
  |
1 | fn bar() -> u8 + 'static {
  |             ^^ not a trait

warning: trait objects without an explicit `dyn` are deprecated
 --> src/lib.rs:1:13
  |
1 | fn bar() -> u8 + 'static {
  |             ^^^^^^^^^^^^ help: use `dyn`: `dyn u8 + 'static`
  |
  = note: `#[warn(bare_trait_objects)]` on by default
