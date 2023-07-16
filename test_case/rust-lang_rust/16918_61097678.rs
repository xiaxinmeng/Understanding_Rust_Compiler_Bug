 rust
[0u8, 1] == [0u8, 1];
//^~ error: mismatched types: expected `[u8]`, found `[u8, ..2]` (expected unsized array, found array)
[0u8, 1].eq([0u8, 1].as_slice());  // OK
[0u8, 1].eq(&[0u8, 1]);  // OK
