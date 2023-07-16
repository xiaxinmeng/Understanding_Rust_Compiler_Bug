 rust
pub struct Ilbm<'a> { body_data: &'a[u8] }

impl<'a> Ilbm<'a> {
    fn get_bit(buffer: &[u8], i: usize) -> u8 { unimplemented!() }
    fn draw_row(&self) { Self::get_bit(&[0]) }
}

fn main() { }
