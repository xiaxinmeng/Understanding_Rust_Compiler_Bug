 rust
let nums = [
     &[f64::NAN, 1.0], // nan
     &[f64::INFINITY, 1.0], // inf
     &[1e308, 1e308, -1e308], // intermediate overflow, 1e308
     &[1e308, 1e308], // inf
     &[0.0, f64::NEG_INFINITY]]; // -inf
for v in nums.iter() {
    println!("{}", v.sum());
}
