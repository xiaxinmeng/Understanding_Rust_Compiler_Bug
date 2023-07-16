rust
pub static BIG_UNINIT: MaybeUninit<[u128; 1 << 27]> = MaybeUninit::uninit();
