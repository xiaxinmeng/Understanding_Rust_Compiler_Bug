rust
fn main() {
    let arr: &[u8; 5] = &[0, 0, 0, 0, 0];
    let value = arr[100];
    println!("{}", value);
}
