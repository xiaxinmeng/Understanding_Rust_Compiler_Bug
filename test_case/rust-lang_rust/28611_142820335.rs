
pub fn main() {
    println!("{}", 2.0f32.powi(4));
}

#[no_mangle]
pub extern fn __powisf2(mut a: f32, mut b: i32) -> f32
{
    let recip = b < 0;
    let mut r = 1f32;
    loop {
        if b & 1 == 1 { r *= a; }
        b /= 2;
        if b == 0 { break; }
        a *= a;
    }
    if recip { 1f32/r } else { r }
}

#[no_mangle]
pub extern fn __powidf2(mut a: f64, mut b: i32) -> f64
{
    let recip = b < 0;
    let mut r = 1f64;
    loop {
        if b & 1 == 1 { r *= a; }
        b /= 2;
        if b == 0 { break; }
        a *= a;
    }
    if recip { 1f64/r } else { r }
}
