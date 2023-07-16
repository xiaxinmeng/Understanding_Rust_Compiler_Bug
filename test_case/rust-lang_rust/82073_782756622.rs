rust
#![feature(let_chains)]
fn main() {
    if let Some(_) = 2 && let Some(sixteen) = 16 {
        let _ = sixteen;
    }
}
