rust
impl<A, F: Fn<A> + ?Sized> FnOnce<A> for Rc<F> {...}
impl<A, F: Fn<A> + ?Sized> FnMut<A> for Rc<F> {...}
impl<A, F: Fn<A> + ?Sized> Fn<A> for Rc<F> {...}

impl<A, F: Fn<A> + ?Sized> FnOnce<A> for Arc<F> {...}
impl<A, F: Fn<A> + ?Sized> FnMut<A> for Arc<F> {...}
impl<A, F: Fn<A> + ?Sized> Fn<A> for Arc<F> {...}
