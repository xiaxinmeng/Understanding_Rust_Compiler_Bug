rust
#[derive(PartialOrd, PartialEq)]
struct MyFloat(f64);
fn main() {
    println!("{}", (0.0 / 0.0 >= 0.0) == (MyFloat(0.0 / 0.0) >= MyFloat(0.0)))
}
