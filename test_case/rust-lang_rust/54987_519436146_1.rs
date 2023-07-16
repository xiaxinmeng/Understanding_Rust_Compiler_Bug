rust
let mut x = MaybeUninit::uninit();
construct_pair_inplace(&mut x,
    || 0i32,
    || panic!(),
);
