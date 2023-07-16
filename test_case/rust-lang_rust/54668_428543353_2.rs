rust
let mut t: mem::MaybeUninit<Block> = mem::MaybeUninit { uninit: () };
let mut t: mem::MaybeUninit<Block> = mem::uninitialized();
