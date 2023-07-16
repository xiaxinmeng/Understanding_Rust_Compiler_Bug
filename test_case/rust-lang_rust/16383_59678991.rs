 rust
#![feature(asm)]

fn main() {
    let mut foo = 42u;
//  unsafe { asm!("leaq $0, %rax" : : "m"(foo) : "rax"); }
    unsafe { asm!("pushq $0; popq $0" : "+g"(foo) ); }
}
