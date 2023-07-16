 rust
pub mod tag {
    pub enum Add {}
    pub enum Mul {}
}

trait OpSelf { type LHS; type RHS; }
impl<LHS, RHS> OpSelf for (LHS, RHS) { type LHS = LHS; type RHS = RHS; }

trait Op<Tag, LHS, RHS>: OpSelf<LHS = LHS, RHS = RHS> { type Out; }

trait Add<LHS, RHS>: Op<tag::Add, LHS, RHS> {
    fn add(self) -> <Self as Op<tag::Add, LHS, RHS>>::Out;
}

struct BigFloat;
impl Op<tag::Add, f64, BigFloat> for (f64, BigFloat) { type Out = BigFloat; }
impl Add<f64, BigFloat> for (f64, BigFloat) {
    fn add(self) -> BigFloat { BigFloat }
}

impl Op<tag::Add, BigFloat, f64> for (BigFloat, f64) { type Out = BigFloat; }
impl Add<BigFloat, f64> for (BigFloat, f64) {
    fn add(self) -> BigFloat { BigFloat }
}

struct MyVec<U>;
impl<U, T: Iterator<Item = U>> Op<tag::Add, T, MyVec<U>> for (T, MyVec<U>) { type Out = MyVec<U>; }
impl<U, T: Iterator<Item = U>> Add<T, MyVec<U>> for (T, MyVec<U>) {
    fn add(self) -> MyVec<U> { unimplemented!() }
}
