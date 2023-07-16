 rust
#![no_main]

#[no_mangle]
pub fn main(args: Vec<String>) {
    for arg in args {
        println!("{}", arg);
    }
}
