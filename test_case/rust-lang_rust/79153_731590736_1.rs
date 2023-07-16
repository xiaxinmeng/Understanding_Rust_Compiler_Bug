console
$ cat b.rs 
#![feature(asm)]
#![feature(naked_functions)]

#[naked]
pub unsafe fn f(a: usize, b: usize) -> ! {
    asm!("/*{0}*//*{1}*/", in(reg) a, in(reg) b);
    unreachable!()
}
$ rustc --crate-type=lib b.rs -O
terminated by signal SIGSEGV (Address boundary error)
