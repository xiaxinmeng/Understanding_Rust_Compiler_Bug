 rust
fn smash<T>(x: Result<u32, T>) -> T {
    x.unwrap_err()
}

let x: Result<u32, !> = Ok(23);
smash(x);
println!("dead code NOT detected");
