rust
#![feature(const_swap)]
#![feature(const_mut_refs)]

#[repr(C, packed)]
struct Demo(u32, &'static i32, u32, i64, i64);

const C: (Demo, Demo) = {
    let mut x = Demo(0, &1, 2, -1, -1);
    let mut y = Demo(3, &4, 5, -1, -1);
    std::mem::swap(&mut x, &mut y);
    (x, y)
};

fn main() {
    let (d1, d2) = C;
}
