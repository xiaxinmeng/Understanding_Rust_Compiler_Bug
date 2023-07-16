rust
#[repr(C)]
struct Demo(u64, u32, u64, u32, u64, u32, u64);

fn main() {
    let mut x = Demo(1, 2, 3, 4, 5, 6, 7);
    let mut y = Demo(10, 11, 12, 13, 14, 15, 16);
    std::mem::swap(&mut x, &mut y);
}
