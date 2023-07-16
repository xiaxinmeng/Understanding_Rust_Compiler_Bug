
pub unsafe fn bar() -> Outer {
    let mut inner = MaybeUninit::uninit();
    
    init(inner.as_mut_ptr());

    Outer { inner: inner.assume_init() }
}
