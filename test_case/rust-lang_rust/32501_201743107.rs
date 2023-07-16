 rust
fn main() {
    let s = String::new();
    let _ = s;  // no op
    let _t = s; // move
    let _u = s; // ERROR: use of moved value
}
