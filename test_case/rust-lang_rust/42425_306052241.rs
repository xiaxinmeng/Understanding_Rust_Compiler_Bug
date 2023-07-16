rust
fn main() {
    let nums = [::std::f64::NEG_INFINITY, -1.0, -0.0, ::std::f64::NAN, 0.0, 1.0, ::std::f64::INFINITY];
    for &num in &nums[..] {
        println!("Number: {}\tSign negative: {}\tSign positive: {}", num, num.is_sign_negative(), num.is_sign_positive());
    }
}
