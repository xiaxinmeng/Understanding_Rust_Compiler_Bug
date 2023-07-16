
#![feature(thread_local)]

#[thread_local]
static FOO: usize = 0;
static BAR: &'static usize = &FOO;

fn main() {
    println!("{}", *BAR);
}

