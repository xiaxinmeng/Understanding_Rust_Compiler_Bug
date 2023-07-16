 rust
///////////////
// HRTB setup
//
trait A {
    type S;
}
trait B<'self_> {
    type T;
}
trait C: for<'self_> B<'self_> {
    type U: for<'self_> A<S=<Self as B<'self_>>::T>;  // requires normalizing through an HRTB
}

//////////////////////
// Problematic impls
//
impl A for usize {
    type S = usize;
}
impl<'self_> B<'self_> for usize {
    type T = usize;
}
impl C for usize {
    type U = usize;
}


fn main() {}
