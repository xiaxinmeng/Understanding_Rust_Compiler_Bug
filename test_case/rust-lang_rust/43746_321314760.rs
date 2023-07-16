rust
#![feature(thread_local)]

#[thread_local]
static A: u32 = 1;

static B: &'static u32 = &A;

fn main() {}
