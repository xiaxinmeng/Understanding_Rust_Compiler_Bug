rust
byte &= !(0x20 * (b'a' <= byte && byte <= b'z') as u8)
