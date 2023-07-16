rust
fn main() {
    let numer = -0.7500000000000006f64;
    let denom = 0.0833333333333334f64;
    let div_euclid = -10.0;
    let rem_euclid = 0.0;
    assert!(numer.div_euclid(denom) - div_euclid < f64::EPSILON);
    assert!(numer.rem_euclid(denom) - rem_euclid < f64::EPSILON);
    assert!(div_euclid * denom + rem_euclid - numer < f64::EPSILON);
}
