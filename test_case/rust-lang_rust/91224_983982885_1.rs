rust
let mut byte = MaybeUninit::uninit();
let input = 42u8;
asm!(
    "st {}, {}",
    in(x) byte.as_mut_ptr() as u16,
    in(reg) input,
);
