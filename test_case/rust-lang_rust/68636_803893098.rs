rust
// Same functions as Fold<B>
trait FoldB {
    fn fold(&mut self, node: B) -> B;
}

// Auto-implement FoldB for Fold<B>
impl<T> FoldB for T where T: Fold<B> {
    fn fold(&mut self, node: B) -> B {
        Fold::<B>::fold(self, node)
    }
}

impl Fold<A> for dyn FoldB {
    // works!
}
