rust
use std::io::{IoSlice, Write};

let mut writer = Vec::new();
let buf = [
    IoSlice::new(b"GET "),
    IoSlice::new(&[47]),
    IoSlice::new(b" HTTP/1.1"),
];
assert_eq!(writer.write_vectored(&buf).unwrap(), 14);
assert_eq!(writer, b"GET / HTTP/1.1");
