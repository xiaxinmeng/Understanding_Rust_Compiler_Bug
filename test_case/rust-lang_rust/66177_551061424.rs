
#[no_mangle]
pub extern fn main(a: i32, b: i32) {
    let z = a + b;
    println!("The value of x is: {}", z);
}
