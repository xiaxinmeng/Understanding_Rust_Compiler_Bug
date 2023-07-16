rust
impl Waker {
    pub unsafe fn from_raw_ref(raw: &RawWaker) -> &Waker { transmute(raw) }
}
