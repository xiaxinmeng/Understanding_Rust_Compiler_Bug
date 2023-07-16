rust
enum Void {}

fn main() {
    let mut x: (Void, [u8; 20]);
    x.1 = [0x12; 20];
}
