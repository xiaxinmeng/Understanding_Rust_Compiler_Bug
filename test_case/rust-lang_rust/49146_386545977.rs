rust
#![feature(const_fn)]
const fn read<T: Copy>(x: &T) -> T { *x }
static FOO: u32 = read(&BAR);
static BAR: u32 = 5;
fn main() {
    println!("{}", FOO);
}
