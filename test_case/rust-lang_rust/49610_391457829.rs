rust
macro_rules! validate_expr {
  ($t:expr) => { }
}

macro_rules! gen_expr {
  () => { validate_expr! { { let try = 22; } }
}
