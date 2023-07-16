rust
const BAD: () = {
  MaybeUninit::<!>::uninit().assume_init();
}
