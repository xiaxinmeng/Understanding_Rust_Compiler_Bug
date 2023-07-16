
impl<T> ArrayTrait for [T; $N] {} // with macros, for 0 <= $N <= 32, one-time bloat
impl<T> BufRead for T where T: ArrayTrait {} // no bloat
