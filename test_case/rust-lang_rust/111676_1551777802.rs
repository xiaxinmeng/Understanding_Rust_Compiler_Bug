rust
    let mut x = Box::new([0_u8; 4096]);
    let y: &mut [u8] = &mut *x;
