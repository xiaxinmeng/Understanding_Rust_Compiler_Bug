rust
fn sad_iter(a: &[u8; 8], b: &[u8; 8]) -> u32 {
  a.iter().zip(b).map(|&a, &b| a.abs_diff(b) as u32).sum()
}

fn sad_loop(a: &[u8; 8], b: &[u8; 8]) -> u32 {
  let mut sum = 0;
  for i in 0..a.len() {
      sum += a[i].abs_diff(b[i]) as u32;
  }
  sum
}
