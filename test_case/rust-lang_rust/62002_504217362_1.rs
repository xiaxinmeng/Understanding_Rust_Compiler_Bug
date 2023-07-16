rust
// whether or not this function is included will change what is printed
// this is analogous to including `std` or not
#[no_mangle]
pub extern "C" fn __panic_internal() {
    println!("the one from std");
}

fn main() {
    // prints "the one from std" or "the one from core" based on whether or not the
    // above function is commented out
    corelike::function_in_core();
}
