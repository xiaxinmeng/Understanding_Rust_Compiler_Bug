rust
fn main() {
    for i in -100..100 {
        putpixel(i);
    }
}

fn putpixel(x: i16) {
    let coordx = (960+x) as u32;
    println!("{coordx}");
}
