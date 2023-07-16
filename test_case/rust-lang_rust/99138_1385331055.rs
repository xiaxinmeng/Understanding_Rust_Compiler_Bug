rust
    assert_eq!(format!("{:x?}", (10, 20)), "(a, 14)");
    assert_eq!(format!("{:#x?}", (10, 20)), "(\n    0xa,\n    0x14,\n)");
