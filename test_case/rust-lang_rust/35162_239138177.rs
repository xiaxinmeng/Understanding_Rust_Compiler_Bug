 rust
let x: Result<u32, !> = Ok(23);
Result::<u32, !>::unwrap_err(x);
println!("dead code detected");
