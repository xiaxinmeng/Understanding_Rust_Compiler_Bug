
impl<'s,     T, R<covariant context>> Coercible<&'s R<&'s T>>         for &'s R<~T> { }
impl<'s,     T, R<covariant context>> Coercible<&'s mut R<&'s mut T>> for &'s mut R<~T> { }
impl<'s, 't, T, R<covariant context>> Coercible<&'s R<&'t T>>         for &'s R<&'t mut T> { }
