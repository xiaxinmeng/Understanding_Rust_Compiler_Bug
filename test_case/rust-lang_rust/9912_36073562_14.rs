
impl<A, B: HasPrefix<A>, C: HasPrefix<B>> HasPrefix<A> for C { }
impl<A, B: Coercible<A>, C: Coercible<B>> Coercible<A> for C { }
