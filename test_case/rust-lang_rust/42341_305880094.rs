rust
struct Closure<T> {
    x: T,
}

impl<'a, T, U> FnOnce<(&'a U,)> for Closure<T>
    where T: Add<U>,
          U: Copy + 'a,
{
    type Output = <T as Add<U>>::Output;
    extern "rust-call" fn call_once(self, (&y,): (&'a U,)) -> Self::Output {
        self.x + y
    }
}

foo(Closure{x: &x})
