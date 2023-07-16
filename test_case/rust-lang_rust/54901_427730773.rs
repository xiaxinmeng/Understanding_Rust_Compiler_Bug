rust
#![feature(thread_local)]

#[thread_local]
static mut S: &str = "before";

fn set_s() {
    unsafe { S = "after" };
}

fn main() {
    println!("{}", unsafe { S });
    set_s();
    println!("{}", unsafe { S });
}
