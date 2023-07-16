rust
fn foo<T>() -> [MaybeUninit<T>; 4] {
    const UNINIT_T: MaybeUninit<T> = MaybeUninit::uninit(); // E0401
    [UNINIT_T; 4]
}
