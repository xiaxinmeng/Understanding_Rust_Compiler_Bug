rust
#![feature(llvm_asm)]
pub struct S;
pub fn f() {
    unsafe { llvm_asm!( "call [rsi + 8*rax]" : : "rbx"(&S) : : "intel" ) };
}
