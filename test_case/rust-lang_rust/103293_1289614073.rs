Rust
fn main() {
    fn is_empty(s: &str) -> bool {
        s.is_empty()
    }
    // Doesn't fail if provided with this
    //let check_val: fn(&str) -> bool = |v| is_empty(v);
    let check_val = |v| is_empty(v);
    let n = "Hi";
    if check_val(&n.to_string()) || check_val("Hello") {}
}
