rust
fn main() {
    let x = #[cfg(test)] 4; // error: removing an expression is not supported in this position
}
