rust
impl RawVec<T, A>
    fn with_capacity(n: usize) -> RawVec<T, HeapAlloc>;
    fn with_capacity_in(n: usize, a: A) -> RawVec<T, A>;
}
