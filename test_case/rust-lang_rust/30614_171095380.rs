 rust
assert_eq!(CStr::from_bytes(slice).unwrap().to_bytes(), slice);
assert_eq!(CStr::from_bytes_unchecked(slice).to_bytes(), slice);
