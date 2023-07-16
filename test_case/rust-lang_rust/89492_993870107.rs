rust
u32vec.iter().fold(0, |acc, x| acc + (*x as i32 - median as i32).abs() as u32)
