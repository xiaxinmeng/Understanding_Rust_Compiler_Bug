rust
impl<A, F: FnOnce<A> + ?Sized> FnOnce<A> for Box<F> { .. }
impl<A, F: FnMut<A> + ?Sized> FnMut<A> for Box<F> { .. }
impl<A, F: Fn<A> + ?Sized> Fn<A> for Box<F> { .. }
