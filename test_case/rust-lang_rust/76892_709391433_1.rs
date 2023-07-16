rust
struct PrivateKey([u8; 32]);
impl PrivateKey {
    pub unsafe fn volatile_zero(&mut self) {
        let ptr = self as *mut Self as *mut [u8; size_of::<PrivateKey>()];
        ptr.write_volatile([0u8; size_of::<PrivateKey>()]);
    }
}
