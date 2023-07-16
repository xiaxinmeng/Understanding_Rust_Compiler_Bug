
trait A {
    type Output;
    fn a(&self) -> <Self as A>::X;
}
