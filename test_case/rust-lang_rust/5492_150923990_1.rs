
#[enum_explicit_descriptor_type(u8)]
#[enum_explicit_descriptor_typeoffset(-1)] // This could be behind-the-struct by default
#[enum_explicit_values = "I: 0, N: 1"]
enum IntOrFloat{
  I(i32),
  N(f32),
}
