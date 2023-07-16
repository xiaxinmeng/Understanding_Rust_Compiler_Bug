
enum Result<Result<u32, u32>, u32>  {
  Ok({ tag: 0u32 | 1u32, payload: u32 }),
  Err(u32),
}
