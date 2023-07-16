rust
#[automatically_derived]
impl<T: ::core::clone::Clone> ::core::clone::Clone for TypedAddress<T> {
    #[inline]
    fn clone(&self) -> TypedAddress<T> {
        TypedAddress {
            inner: ::core::clone::Clone::clone(&self.inner),
            phantom: ::core::clone::Clone::clone(&self.phantom),
        }
    }
}
#[automatically_derived]
impl<T: ::core::marker::Copy> ::core::marker::Copy for TypedAddress<T> { }
