
error[E0277]: the trait bound `<V as VectorSpace>::Scalar: Real` is not satisfied
  --> src/lib.rs:13:1
   |
7  | / pub trait InnerSpace: VectorSpace where
8  | |     <Self as VectorSpace>::Scalar: Real,
9  | | {
10 | |     fn dot(self, other: Self) -> Self::Scalar;
11 | | }
   | |_- required by `InnerSpace`
12 | 
13 |   pub fn dot<V: InnerSpace>(a: V, b: V) -> V::Scalar {
   |   ^                                                 - help: consider further restricting the associated type: `where <V as VectorSpace>::Scalar: Real`
   |  _|
   | |
14 | |     V::dot(a, b)
15 | | }
   | |_^ the trait `Real` is not implemented for `<V as VectorSpace>::Scalar`
