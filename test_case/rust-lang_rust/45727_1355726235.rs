rust
fn main() {
    let _ = (-10..=10).find(|x: i32| x.signum() == 0);
}
