rust
impl<A, F: Fn<A> + ?Sized> FnOnce<A> for &F {...}
impl<A, F: Fn<A> + ?Sized> FnMut<A> for &F {...}
impl<A, F: Fn<A> + ?Sized> Fn<A> for &F {...}
