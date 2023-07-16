 rust
fn main() {
    let test_val: f64 = 546.54;
    assert_eq!(test_val.floor(), 546.0);
    assert_eq!(test_val.min(0.0), 0.0);
    println!("ok");
}
