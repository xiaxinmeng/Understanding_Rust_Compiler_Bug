rust
fn main() {
    let closure = |_v| {};
    
    let a = vec!["".to_string(), "".to_string(), "".to_string()];
    for b in a {
        closure(&b);
    }
}
