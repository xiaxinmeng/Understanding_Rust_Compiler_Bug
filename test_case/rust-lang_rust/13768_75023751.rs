 rust
fn main() {                          
    const N: u8 = 0;
    const M: u8 = 1;
    const O: u8 = N - M;
    const P: usize = O;
    let _foo: [u8; 255] = [0; P];
}
