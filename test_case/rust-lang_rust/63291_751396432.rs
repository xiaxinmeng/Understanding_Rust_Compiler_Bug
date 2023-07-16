rust
let mut rc_uninit: Rc<MaybeUninit<NonNull<()>>> = Rc::new(MaybeUninit::new(t));
let rc: Rc<NonNull<()>> = unsafe {
    // SAFETY:
    // > As with MaybeUninit::assume_init, it is up to the caller to guarantee
    //   that the inner value really is in an initialized state.
    // The contents of rc_uninit are initialized.
    rc_uninit.clone().assume_init()
};
unsafe {
    // SAFETY:
    // > Any other `Rc` or [`Weak`] pointers to the same allocation must not be
    //    dereferenced for the duration of the returned borrow.
    // The borrow only lasts for this unsafe block and no other Rc is dereferenced.
    *Rc::get_mut_unchecked(&mut rc_uninit) = MaybeUninit::uninit();
}
