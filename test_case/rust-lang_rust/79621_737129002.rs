rust
const TRUE: bool = {
  let mut x = MaybeUninit::<bool>::uninit();
  x.as_mut_ptr().write(true);
  x.assume_init()
};
assert!(TRUE);
