`rust
pub fn main() {
    let x = String::from("hello world");
    let _ = x.replace("x", &x);
}
