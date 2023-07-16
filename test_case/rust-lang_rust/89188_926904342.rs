
impl<T: ::core::clone::Clone + CallWithShim + 'static> ::core::clone::Clone for ShimMethod<T>
where
    for<'s> T::Shim<'s>: ::core::clone::Clone,
//  ^^^^^^^-------- insert here?
{
    #[inline]
    fn clone(&self) -> ShimMethod<T> {
        match *self {
            ShimMethod(ref __self_0_0) => ShimMethod(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
