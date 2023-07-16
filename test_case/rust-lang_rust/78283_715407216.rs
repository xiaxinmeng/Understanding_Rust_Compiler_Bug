rust
fn inner() -> Vec<u8> {
    let color_sample_max = 4;
    let pixel_size = 4;
    let mut out = vec![0; color_sample_max * 256 * pixel_size];

    let mut color = &mut out[..];
    for _ in 0..color_sample_max {
        for b in 0..=255 {
            color[0] = 0x80;
            color[1] = 0x80;
            color[2] = b;
            color[3] = 0x80;

            color = &mut color[pixel_size..];
        }
    }
    out
}

fn f1()  {
    inner();
}

fn f2()  {
    inner();
}

fn main() {
    f1();
    f2();
}
