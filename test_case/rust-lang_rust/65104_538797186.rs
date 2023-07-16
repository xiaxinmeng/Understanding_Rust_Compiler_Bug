rust
fn main() {
    let mut s: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let n: i8 = -3;
    s.rotate_right(n.rem_euclid(s.len()) as usize);
    println!("{:?}", s);
}
