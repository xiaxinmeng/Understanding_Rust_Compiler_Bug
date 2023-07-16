rust
let mut i = 1.0;
let mut sum = 0.0f32;
while i < 40_000_000.0 {
    sum += 1.0 / i - 1.0 / (i + 2.0);
    i += 4.0;
}
println!("pi = {}", 4.0 * sum);
