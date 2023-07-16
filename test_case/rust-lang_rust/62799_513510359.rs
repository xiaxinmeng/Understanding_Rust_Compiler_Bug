rust
impl<T> MaybeUninit<T> {
    #[unstable(
        feature = "internal_uninit_const",
        issue = "0",
        reason = "hack to work around promotability",
    )]
    pub const UNINIT: Self = Self::uninit();
}

fn foo<T>() {
    let x = [MaybeUninit::<T>::UNINIT; 5]; // OK.
}
