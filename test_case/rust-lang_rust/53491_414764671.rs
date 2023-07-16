rust
let mut b = MaybeUninit::<u8>::uninitialized().into_inner();
let bref = &mut b; // Insta UB ?
