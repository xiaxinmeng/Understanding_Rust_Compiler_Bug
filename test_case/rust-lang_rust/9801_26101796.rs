 rust
// A cast from a signed value widens with signed-extension
(-1i8 as i16, -1i8 as u16) // (-1i16, 65535u16)
// A cast from an unsigned value widens with zero-extension
(1u8 as i16, 1u8 s u16) // (1, 1)
