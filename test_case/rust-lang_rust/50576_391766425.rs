rust
fn main() {
    let x: [u8; {
        let mut x = 42;
        while x < 50 {
            x += 7;
        }
        x
    }] = [0; 0];
}
