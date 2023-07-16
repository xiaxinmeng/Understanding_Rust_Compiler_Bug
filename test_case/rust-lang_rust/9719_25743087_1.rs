 Rust
...
fn main() {
    let x = 42;
    let _y = Y { x: Some(&x as &X) }; // inlining a
}
