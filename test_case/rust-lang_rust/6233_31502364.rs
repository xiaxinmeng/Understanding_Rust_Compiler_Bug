 rust
#[feature(thread_local)];
#[allow(dead_code)];

#[cfg(crash)]
static SIZE: uint = 1 << 20; // 1 MB

#[cfg(not(crash))]
static SIZE: uint = 1 << 10; // 1 KB

#[thread_local]
static FOO: [u8, .. SIZE] = [0, .. SIZE];

fn main() {}
