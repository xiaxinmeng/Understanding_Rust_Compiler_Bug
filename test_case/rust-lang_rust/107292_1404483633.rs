rust
fn main() {
    let arr: [u32; 3] = [0, 1, 2];
    let idx = 0u8;
    let val: u32 = arr[idx.into()];
    println!("{}", val);
}
