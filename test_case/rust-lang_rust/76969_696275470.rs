rust
#[repr(transparent)]
pub struct UnixFd(pub raw::c_int); // or even just `Fd`
