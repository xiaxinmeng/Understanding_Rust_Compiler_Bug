 rust
fn main() {
    let a = from_str::<f32>("3.141592").unwrap();
    let b = 3.141592f32;
    println!("{}", a == b); // false
}
