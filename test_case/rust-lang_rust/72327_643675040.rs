rust
#[inline(never)]
fn get_num() -> f64 {
    let num: f64 = 1.0e30;
    // volatile is to avoid optimizations
    unsafe { std::ptr::read_volatile(&num) }
}

fn main() {
    let x = get_num();
    let y = get_num();
    let z = x * y;
    if z != f64::INFINITY && z != f64::NEG_INFINITY && !z.is_nan() {
        let exp = ((z as f32).to_bits() >> 23) & 0x7F;
        assert!(exp != 0x7F);
        println!("is finite, exp = {}", exp);
    } else {
        println!("is not finite");
    }
}
