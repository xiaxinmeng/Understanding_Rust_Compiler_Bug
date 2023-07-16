rust
impl<T: AsFd> AsFd for &T {
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}
impl<T: AsFd> AsFd for &mut T {
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}
