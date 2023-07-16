
fn transpose_with_const<const W: usize, const H: usize>(
  v: [[u32; 2 * H]; W + W]
) -> [[u32; W + W]; 2 * H]
