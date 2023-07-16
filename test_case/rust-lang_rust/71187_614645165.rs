rust
    let raw_data = &[0x78, 0x56, 0x34, 0x12];
    
    let num = unsafe {
        std::mem::transmute::<[u8; 4], u32>(*raw_data)
    };
    // This result depends on your machine
    assert_eq!(num, 0x12345678);


    // use from_be_bytes and from_le_bytes instead to produce predictable result
    let num = u32::from_le_bytes(*raw_data);
    assert_eq!(num, 0x12345678);
    
    let num = u32::from_be_bytes(*raw_data);
    assert_eq!(num, 0x78563412);
