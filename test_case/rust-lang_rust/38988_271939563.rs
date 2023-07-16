
fn main() {
    println!("Hello, world!");

    unsafe {
        asm!("\
            call $0
            " : : "i"(other_function as extern "C" fn())::"intel"
        );
    }
}
