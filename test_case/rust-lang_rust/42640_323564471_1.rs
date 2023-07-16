rust
const CONST_REF: &[u8; 2] = &[1u8, 2u8];

fn main() {
    let f: &[u8; 2] = &[2u8, 3u8];
    match f {
        CONST_REF => (),
        _ => (),
    };
}
