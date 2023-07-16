
#![feature(asm)]

fn boom() {
    ::std::process::exit(42)
}

fn main() { unsafe {
    asm!("jmpq *$0" : : "r"(boom as fn()));
} }
