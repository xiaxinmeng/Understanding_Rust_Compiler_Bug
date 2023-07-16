rust
<Self as Fn(A) -> B>::call
// Desugars to
<Self as Fn<(A,), Output = B>>::call
