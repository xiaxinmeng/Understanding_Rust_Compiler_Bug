rust
const ZST: &[u8] = unsafe { std::mem::transmute(1usize) };
