rust
const CONST_REF: &[u8; 3] = b"foo";

fn main() {
    let f = b"bar";
    match f {
        CONST_REF => (),
        _ => (),
    };
}
