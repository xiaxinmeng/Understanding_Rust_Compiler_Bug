 rust
fn main() {
    let mut s = String::new();
    s.push_str("0.");
    for _ in 0..370 {
        s.push_str("3");
    }
    s.parse::<f64>();
}
