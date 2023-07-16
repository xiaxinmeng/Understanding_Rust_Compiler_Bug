rust
// core::array

fn try_map<U, R, F>(self, f: F) -> R::TryType<[U; N]>
where
    R: Residual,
    F: FnMut(T) -> R::TryType<U>,
{ .. }
