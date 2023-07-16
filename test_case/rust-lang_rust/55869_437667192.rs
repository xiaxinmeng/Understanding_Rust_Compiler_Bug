
pub fn iterate<T, S, F: FnMut(S) -> Option<(T, S)>>(state: S, f: F) -> Iterate<F>;
