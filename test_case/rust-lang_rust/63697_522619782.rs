rust
#[derive(Debug)]
struct Member(u16);

pub fn main() {
    let m = Member(i32::from(0_u8));
    println!("{:?}", m);
}
