rust
#![feature(min_const_generics)]

pub struct Buffer<T: Default + Copy, const S: usize> {
    contains: [T; S],
}

pub const T_WIDTH: usize = 80;
pub const T_HEIGHT: usize = 25;
pub const T_SIZE: usize = T_WIDTH * T_HEIGHT;

#[derive(Clone, Copy, Default)]
pub struct VgaChar(char);

pub fn vga_text_swap(buf: &Buffer<VgaChar, T_SIZE>) {}

fn main() {
    let buf = &Buffer {
        contains: [VgaChar('a'); 2000],
    };
    vga_text_swap(buf as &Buffer<VgaChar, 2000>);
}
