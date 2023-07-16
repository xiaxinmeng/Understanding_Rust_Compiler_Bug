
fn i2f(i: u8) -> f64 {(i as f32)/127f32 - 1f32}
println!("{:?}", i2f(125u8));
