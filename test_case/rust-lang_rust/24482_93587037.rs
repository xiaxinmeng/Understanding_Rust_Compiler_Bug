 rust
fn main() {
    let p = ("x".to_string(), "y".to_string());
    let (_, ref y) = p;
    let (x, _) = p;
    println!("{} {}", x, y);
}
