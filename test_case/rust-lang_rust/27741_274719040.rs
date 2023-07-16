rust
pub struct ExclusiveRange<T, S> {
    start: T,
    stop: T,
    step: S,
}

pub struct InclusiveRange<T, S> {
    start: T,
    stop: T,
    step: S,
}

pub struct UnboundedRange<T, S> {
    start: T,
    step: S,
}
