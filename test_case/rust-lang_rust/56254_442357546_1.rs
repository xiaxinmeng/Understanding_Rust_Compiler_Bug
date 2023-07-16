rust
fn fake_mut<'a, 'b, T>(x: &'a &'b T) -> &'a &'b mut T {
  std::mem::transmute(x)
}
