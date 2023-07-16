 rust
fn main() {                          
    const N: u8 = 0 - 1;            
    let _foo: [u8; 255] = [0; N as usize];
}
