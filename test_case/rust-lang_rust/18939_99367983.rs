 rust
static FOO: [usize; 2] = [0, 0];
static BAR: &'static mut [usize; 2] = & FOO;

fn main() {}
