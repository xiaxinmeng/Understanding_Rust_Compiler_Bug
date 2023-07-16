
impl<U, T: Coercible<U>, R<covariant context>> Coercible<R<U>> for R<T> { }
