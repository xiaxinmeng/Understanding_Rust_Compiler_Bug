rust
// Finally, and most oddly, using an identity cast or type ascription
// from `u32` to `u32` also convinces the inference engine:
let c: &u32 = &5; ((c >> 8) as u32 & 0xff) as u8;
