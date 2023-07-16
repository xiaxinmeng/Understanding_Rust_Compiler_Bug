rust
impl<I> Meow for I
    where I: Iterator
{
    existential type MeowType: Iterator<Item = I::Item>;
    fn meow(self) -> Self::MeowType {
        self
    }
}
