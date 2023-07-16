
#[enum_explicit_descriptor(t)]
#[enum_explicit_values = "I: 0, N: 1"]
unsafe struct TValue{
  t: u8,
  val: unsafe enum IntOrFloat{
    I(i32),
    N(f32),
  },
}
