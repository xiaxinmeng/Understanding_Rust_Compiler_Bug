 rust
use std::num::strconv;

fn main() {
    let n: Option<u8> = strconv::from_str_bytes_common(b"12", 10, false, false, false, strconv::ExpNone, false, false);
    println!("{}", n);

    let n: Option<u8> = strconv::from_str_bytes_common(&[0, 0xFF], 10, false, false, false, strconv::ExpNone, false, false);
    println!("{}", n);
}
