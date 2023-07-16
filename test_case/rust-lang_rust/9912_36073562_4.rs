
impl<T, static N: uint> Coercible<[T, ..N]> for (T, T, .. times N) { } // fake syntax
impl<T, static N: uint> Coercible<(T, T, .. times N)> for [T, ..N] { }
