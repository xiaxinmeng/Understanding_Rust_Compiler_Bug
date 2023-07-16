rust
// in src/lib.rs

mod this_is_inline {
  #[path = "../support.rs"]
  mod support;
}
