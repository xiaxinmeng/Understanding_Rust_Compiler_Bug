 rust
let x: Result<u32, !> = Ok(23);
Result::unwrap_err(x);
println!("dead code NOT detected");
