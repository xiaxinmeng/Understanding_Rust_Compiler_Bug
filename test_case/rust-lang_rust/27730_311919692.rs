rust
#[derive(Copy, Clone)]
pub struct NonZeroUsize(&'static ());

impl NonZeroUsize {
    #[inline]
    pub unsafe fn new(value: usize) -> Self {
        NonZeroUsize(&*(value as *const ()))
    }

    #[inline]
    pub fn get(self) -> usize {
        self.0 as *const () as usize
    }
}
