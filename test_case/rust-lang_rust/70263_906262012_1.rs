rust
pub trait FnOnceShim<'a, T: 'a> {
    type Output: 'a;

    fn call(self, input: T) -> Self::Output;
}

impl<'a, F, In, Out> FnOnceShim<'a, In> for F
where
    F: FnOnce(In) -> Out,
    In: 'a,
    Out: 'a,
{
    type Output = Out;

    fn call(self, input: In) -> Self::Output {
        self(input)
    }
}

impl<'a, R: RawMutex + 'a, T: ?Sized + 'a> MutexGuard<'a, R, T> {
    // ..

    pub fn map<F>(
        s: Self,
        f: F,
    ) -> MappedMutexGuard<'a, R, <F as FnOnceShim<'a, &'a mut T>>::Output>
    where
        for<'any> F: FnOnceShim<'any, &'any mut T>,
    {
        // ..
    }

    // ..
}
