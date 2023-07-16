rust
let tuple: (Option<A>, Option<B>) = ...;
let res: Option<(A, B)> = match tuple {
    (Some(a), Some(b)) => Some((a, b)),
    _ => None,
};
