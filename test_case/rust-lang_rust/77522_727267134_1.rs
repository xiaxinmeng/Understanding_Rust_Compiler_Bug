rust
({
    #[inline]
    fn likely (b: bool) -> bool {
        unsafe { ::std::intrinsics::likely(b) }
    }
    likely
})($e)
