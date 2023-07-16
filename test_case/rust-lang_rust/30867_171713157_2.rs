
impl<T, U, F> Unary<T> for F
    where F: Fn<(T,)>, <F as Fn<(T,)>>::Output = U
