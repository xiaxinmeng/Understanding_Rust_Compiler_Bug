rust
impl<A, B> ZipImpl<A, B> for Zip<A, B> {
    fn try_fold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R where
        Self: Sized, F: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc> {
        self.a.try_fold(...)
    }
}

impl<A, B> ZipImpl<A, B> for Zip<A, B>
    where A: TrustedRandomAccess, B: TrustedRandomAccess
{
    fn try_fold<Acc, F, R>(&mut self, init: Acc, mut f: F) -> R where
        Self: Sized, F: FnMut(Acc, Self::Item) -> R, R: Try<Ok=Acc> {
        // Copy `Iterator::try_fold` implementation here
    }
}
