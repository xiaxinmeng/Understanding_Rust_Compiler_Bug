rust
pub fn iterate<T, F>(
    seed: T,
    f: F,
) -> Unfold<(F, Option<T>, bool), fn(&mut (F, Option<T>, bool)) -> Option<T>>
where
    T: Clone,
    F: FnMut(T) -> T,
