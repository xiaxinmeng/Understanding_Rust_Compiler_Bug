 rust
impl Clone for T where T: Pod {
    fn clone(&self) -> T {
        // NB safe! this a memcpy of Pod data (no owned/refcounted/`drop`able data here)
        unsafe {
            mem::transmute_copy(self)
        }
    }
}
