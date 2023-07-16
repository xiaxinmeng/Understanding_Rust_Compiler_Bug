
fn i2f(i: u8) -> f64 {(i as f64)/127f64 - 1f64}
println!("{:?}", i2f(125u8));
