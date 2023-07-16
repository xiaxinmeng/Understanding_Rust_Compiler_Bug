rust
struct A<'a, T>(&'a T);

impl<'a, T: 'a> Clone for A<'a, T> {
    fn clone(&self) -> Self {
        A(self.0.clone())
    }
}

impl<'a, T: 'a> Copy for A<'a, T> {}

#[derive(Clone)]
struct B<'a, T>(A<'a, T>);

// `T: '_` should be implied by `WF(B<'_, T>)`.
impl<T: Copy> Copy for B<'_, T> {}
