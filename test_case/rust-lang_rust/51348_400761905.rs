rust
#![feature(nll)]

fn foo(x: &mut Result<u32, u32>) {
    match x {
        Ok(v) | Err(v) if *v > 0 => { }
        _ => { }
    }
}

fn main() { }
