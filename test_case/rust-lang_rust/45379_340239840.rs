rust
fn try_for_each<F, R>(&mut self, mut f: F)
    where Self: Sized, F: FnMut(Self::Item) -> R, R: Try<Ok=()>
{
    self.try_fold((), move |(), x| f(x))
}
