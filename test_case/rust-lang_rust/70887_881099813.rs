rust
pub const fn log10_u32_proposed(x: u32) -> u32 {
    const TABLE: &[u64] = &[
        0x0000_0000_0000,
        0x0000_0000_0000,
        0x0000_0000_0000,
        0x0000_FFFF_FFF6,
        0x0001_0000_0000,
        0x0001_0000_0000,
        0x0001_FFFF_FF9C,
        0x0002_0000_0000,
        0x0002_0000_0000,
        0x0002_FFFF_FC18,
        0x0003_0000_0000,
        0x0003_0000_0000,
        0x0003_0000_0000,
        0x0003_FFFF_D8F0,
        0x0004_0000_0000,
        0x0004_0000_0000,
        0x0004_FFFE_7960,
        0x0005_0000_0000,
        0x0005_0000_0000,
        0x0005_FFF0_BDC0,
        0x0006_0000_0000,
        0x0006_0000_0000,
        0x0006_0000_0000,
        0x0006_FF67_6980,
        0x0007_0000_0000,
        0x0007_0000_0000,
        0x0007_FA0A_1F00,
        0x0008_0000_0000,
        0x0008_0000_0000,
        0x0008_C465_3600,
        0x0009_0000_0000,
        0x0009_0000_0000,
    ];
    ((x as u64 + TABLE[31 - x.leading_zeros() as usize]) >> 32) as _
}
