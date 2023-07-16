shell
> cat broken.rs
#![feature(asm)]

fn main() {
    unsafe {
        asm!(" test:")
    }
    println!("HERE")
}

> rustc broken.rs; and ./broken
fish: 'and ./broken' terminated by signal SIGSEGV (Address boundary error)

> opt -O2 broken.ll | llc -O2 -o broken.s; and clang build/x86_64-apple-darwin/stage2/lib/libstd-774948d0982f7640.dylib  broken.s  -o broken
> ./broken
HERE
