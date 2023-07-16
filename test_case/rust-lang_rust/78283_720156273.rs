rust
const color_sample_max: usize = 2;
const pixel_size: usize  = 4;

#[inline(never)]
fn inner(mut out: &mut [u8])  {

    let mut color = &mut out[..];
    for _ in 0..color_sample_max {
        for b in 0..=255 {
            unsafe {
            if color.len() < 1 { panic!() }
            *color.get_unchecked_mut(0) = 0x80;
            if color.len() < 2 { panic!() }
            *color.get_unchecked_mut(1) = 0x80;
            if color.len() < 3 { panic!() }
            *color.get_unchecked_mut(2) = b;
            *color.get_unchecked_mut(3) = 0x80;
            }
            color = &mut color[pixel_size..];
        }
    }
}


fn main() {
    let mut out = vec![0; color_sample_max * 256 * pixel_size];
    let out = inner(&mut out);
}
