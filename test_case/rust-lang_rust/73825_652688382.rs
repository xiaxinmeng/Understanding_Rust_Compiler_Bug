rust
fn foo() {
  let x: [u32; 5] = *&[1,2,3,4,5];
  let y: [u32; 5] = *&[1,2,3,4,5];

  assert_ne!(&x as *const [u32; 5], &y as *const [u32; 5])
}
