rust
// Given to_be/to_le and to_bytes, only one way to put them together:
let _ = word.to_be().to_bytes();
let _ = word.to_le().to_bytes();
// Given from_be/from_le and from_bytes, only one way to put them together:
let _ = u32::from_be(u32::from_bytes(bytes));
let _ = u32::from_le(u32::from_bytes(bytes));
// Cheeky :P
let _ = (u32::from_be . u32::from_bytes)(bytes);
