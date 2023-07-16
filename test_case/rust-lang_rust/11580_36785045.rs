 rust
fn main() {
    let m : [u8, ..1] = [ 0u8 ];
    println!("{}", ((m[0] >> 8) | 0) as u16);
    println!("{}", ((m[0] >> 8) | m[0]) as u16);
    assert!((((m[0] >> 8) | 0) as u16) == 0);
    assert!((((m[0] >> 8) | m[0]) as u16) == 255);
}
