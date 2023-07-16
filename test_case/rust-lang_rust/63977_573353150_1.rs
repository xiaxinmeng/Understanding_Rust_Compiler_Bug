asm
#![feature(asm)]

fn main() {
    let result: u64;
    unsafe { asm!("
        xorl %eax, %eax
    0:
        nop
        addq $$1, %rax
        cmpq $$500, %rax
        jne 0b
    ": "={rax}"(result)
    :
    :
    :) };
    println!("Result: {}", result);
}
