rust
fn main() {
    let x = -180 as i16;
    putpixel(x);
}

fn putpixel(x: i16) {
    let coordx = 960+x as u32;
    println!("{coordx}");
}
