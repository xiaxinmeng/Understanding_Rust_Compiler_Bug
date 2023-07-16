rust
let mut x = MaybeUninit::<[T; N]>::uninit();
for i in &mut x {
    *x = X;
}
let x = x.assert_initialized();
