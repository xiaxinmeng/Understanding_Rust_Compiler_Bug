rust
#[cfg(target_c_char_signed)]
type c_char = i8;
#[cfg(not(target_c_char_signed))]
type c_char = u8;
