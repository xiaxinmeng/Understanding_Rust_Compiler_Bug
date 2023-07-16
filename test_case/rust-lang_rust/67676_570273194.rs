rust
    let x = 1026u16 as u8; // lint fires here
    let y = (1026u16 & 0x00FFu16) as u8; // lint does not fire here
