rust
#![feature(or_patterns)]

fn main() {
    let x: Option<u32> = Some(0);
    let Some(_) | None = x;
}
