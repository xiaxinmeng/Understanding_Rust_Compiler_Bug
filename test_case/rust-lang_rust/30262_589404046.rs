rust
impl<T, U, V, W> PrivatePow<T, UInt<U, V>> for W
    where W: Mul<W> + Mul<T>,
         <W as Mul<W>>::Output: PrivatePow<<W as Mul<T>>::Output, UInt<U, V>>
