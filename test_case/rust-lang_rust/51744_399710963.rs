 rust
#![feature(asm)]
fn main() {
    let input = 42u32;
    let output: u32;
    unsafe {
        asm!("movl $1, $0" : "=r"(output) : "m"(input));
    }
    println!("{}", output);
}
