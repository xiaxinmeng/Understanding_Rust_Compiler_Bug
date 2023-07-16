rust
assert!(format!("{:02x?}", b"Foo\0") == "[46, 6f, 6f, 00]");
assert!(format!("{:02X?}", b"Foo\0") == "[46, 6F, 6F, 00]");
