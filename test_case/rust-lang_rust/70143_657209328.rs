rust
#[repr(align(0x800000))]
struct Aligned(usize);

fn main() {
    let x = Aligned(2);
    println!("{}", x.0);
}
