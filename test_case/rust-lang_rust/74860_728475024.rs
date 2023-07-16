rust
trait FromRawFd {
  fn from_wrapper(other: &'a impl AsRawFd) -> BorrowedFd<'a, Self> {
     BorrowedFd {inner: ManuallyDrop::new(Self::from_raw_fd(other.as_raw_fd())), phantom: PhantomData}
  }
}
