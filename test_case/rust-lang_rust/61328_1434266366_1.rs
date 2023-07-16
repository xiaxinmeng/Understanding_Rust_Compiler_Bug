rust
// main.rs
fn main() {
  println!("{}", std::hint::black_box(45.0f64).exp());
}
