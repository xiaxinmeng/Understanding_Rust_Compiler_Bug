rust
fn as_array(&self) -> [f32; Self::W] {
  let mut result = [0.0; Self::W];
  result[..Self::W].copy_from_slice(&self[..Self::W]);
  result
}
