rust
fn main() {
    let operator = std::f64::NAN > std::f64::NAN;
    let method = std::f64::NAN.gt(&std::f64::NAN);
    println!("{} {}", operator, method); // false false
    assert_eq!(operator, method); // doesn't fail
    assert_eq!(std::f64::NAN > std::f64::NAN, std::f64::NAN.gt(&std::f64::NAN)); // fails
}
