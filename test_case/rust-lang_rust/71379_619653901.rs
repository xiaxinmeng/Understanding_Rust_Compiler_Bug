rust
#[repr(transparent)]
struct MoveRef<'a, T>(&'a mut T);

impl<'a, T> MoveRef<'a, T> {
    pub fn give(r: &'a mut T) -> Self {
        MoveRef(r)
    }

    pub fn take(self) -> T {
        mem::take(self.0)
    }
}
