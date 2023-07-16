rust
let duration = std::time::Duration::new(u64::max_value(), 1_000_000_000-1);
println!("{:?}", duration); // 18446744073709551615.999999999s
