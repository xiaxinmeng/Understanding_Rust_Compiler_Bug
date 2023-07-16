
#![feature(asm)]
fn main() {
    let rax: u64;
    unsafe {asm!("" :"={rax"(rax))};
    println!("Accumulator is: {}", rax);
}
