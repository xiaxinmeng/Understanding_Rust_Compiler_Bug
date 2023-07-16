rust
const BAD : [f32; (&1u8 as *const u8 as usize) % 2] = [4.2];
