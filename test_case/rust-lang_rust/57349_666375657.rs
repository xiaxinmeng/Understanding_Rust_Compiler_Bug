rust
pub struct MutRef<T>(*mut T);

impl<T> MutRef<T> {
    pub const fn new(x: &'static mut T) -> Self {
        Self(x)
    }
    pub fn into_mut_ref(self) -> &'static mut T {
        unsafe {
            &mut *self.0
        }
    }
}
