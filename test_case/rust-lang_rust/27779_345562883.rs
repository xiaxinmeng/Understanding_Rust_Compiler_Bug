rust
pub struct MutOnlyRef<'a, T: 'a> { data: &'a mut T }

impl<'a, T> MutOnlyRef<'a, T> {
    pub fn new(data: &'a mut T) -> Self { Self { data } }

    pub fn set(self, src: T) -> &'a mut T {
        unsafe { ptr::write(self.data, src); }
        self.data
    }
}