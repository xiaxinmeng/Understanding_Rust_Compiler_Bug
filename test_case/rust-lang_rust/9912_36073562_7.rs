
impl<T, static M: uint, static N: uint> where N > M HasPrefix<[T, ..M]> for [T, ..N] { }
