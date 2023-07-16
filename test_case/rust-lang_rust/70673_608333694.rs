rust
#![feature(thread_local)]

#[thread_local]
static A: &u8 = &42;

fn main() {
    dbg!(*A);
}
