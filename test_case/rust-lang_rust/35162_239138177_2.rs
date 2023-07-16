 rust
let x: Result<u32, !> = Ok(23);
x.unwrap_err();
println!("dead code NOT detected");
