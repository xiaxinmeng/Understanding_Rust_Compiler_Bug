rust
#[repr(transparent)]
pub struct Outer {
    inner: Inner,
}

pub unsafe fn foo() -> Outer {
    let mut outer = MaybeUninit::<Outer>::uninit();
    init(outer.as_mut_ptr().cast());
    outer.assume_init()
}
