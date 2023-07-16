rust
fn mk_uninit1<N: const usize>() -> [MaybeUninit<u8>; N] {
  [MaybeUninit::uninit(); N]
}
fn mk_uninit2<N: const usize>() -> [MaybeUninit<u8>; N] {
  MaybeUninit::uninit_array()
}
