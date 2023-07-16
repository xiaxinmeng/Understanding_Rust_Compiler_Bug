rust
pub const INC: usize = 14;
pub const SIZE: usize = 1 << 32 + INC;

fn main() {
    let big_array: [u8; SIZE] = [0; SIZE];
    for n in 0..SIZE {
        println!("{}", big_array[n])
    }
}
