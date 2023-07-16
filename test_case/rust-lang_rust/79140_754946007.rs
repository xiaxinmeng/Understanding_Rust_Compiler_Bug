rust
let mut foo = MaybeUninit::<Vec<()>>::uninit(); // or `zeroed`, but if you plan to overwrite it anyway, you can leave it uninitialized.
