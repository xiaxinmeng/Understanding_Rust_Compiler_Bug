rust
let mut buf = [MaybeUninit::<u8>::uninit(); 20];
let mut buf: BorrowedBuf = buf.as_mut_slice().into();
write!(buf.unfilled(), "{}", 42).unwrap();
let buf: &[u8] = buf.filled();
