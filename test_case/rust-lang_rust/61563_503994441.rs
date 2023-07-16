rust
#[cfg(any(test, feature = "generic-array"))]
impl<N, A> From<generic_array::GenericArray<A, N>> for Vec<N, A>
where
    N: Unsigned + generic_array::ArrayLength<A>,
{
    /// Construct a vector of size `N` from a `GenericArray`.
    ///
    /// # Examples
    /// 