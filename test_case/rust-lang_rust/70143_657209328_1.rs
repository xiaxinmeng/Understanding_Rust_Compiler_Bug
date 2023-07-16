rust
#[repr(align(0x10000000))]
struct Aligned(usize);

fn main() {
    let x = Aligned(2);
    println!("{}", x.0);
}
