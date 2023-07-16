 rust
let mut iter = 'ä'.encode_utf8();
iter.next(); // get the first byte
let x = iter.as_str();
let y = iter.as_slice();
