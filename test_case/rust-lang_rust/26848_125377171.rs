 rust
const fn bmax() -> u8 { !0 }
const fn smax() -> u16 { !0 }
let c: [u8; bmax() as usize] = [0; 255];
let c: [u8; smax() as usize] = [0; 65535];
