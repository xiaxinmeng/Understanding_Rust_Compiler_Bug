
fn generate_nats_from(x_: uint) -> fn@() -> uint {
  let x = ~mut x_;
  || { *x += 1; *x }
}
