rust
fn foo(mut x: Result<u32, &mut u32>) {
    match x {
        Ok(ref mut v) | Err(v) if *v > 0 => { }
        _ => { }
    }
}
