rust
trait MyCopy {
    fn copy(&self) -> Self;
}

impl<'a, T: 'a> MyCopy for &'a T {
    fn copy(&self) -> Self {
        *self
    }
}

struct A<'a, T>(&'a T);

impl<'a, T: 'a> MyCopy for A<'a, T> {
    fn copy(&self) -> Self {
        A(self.0.copy())
    }
}

struct B<'a, T>(A<'a, T>);

// `T: '_` should be implied by `WF(B<'_, T>)`.
impl<T: MyCopy> MyCopy for B<'_, T> {
    fn copy(&self) -> Self {
        B(self.0.copy())
    }
}
