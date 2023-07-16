
error[[E0599]](https://doc.rust-lang.org/stable/error-index.html#E0599): the method `read_bytes` exists for mutable reference `&mut Self`, but its trait bounds were not satisfied
  --> src/lib.rs:21:26
   |
21 |         let bytes = self.read_bytes(size);
   |                          ^^^^^^^^^^
   |
note: trait bound `Self: Sized` was not satisfied
  --> src/lib.rs:15:6
   |
15 | impl<R: ReadBytesExt> BinaryReader for R {}
   |      ^                ------------     -
   |      |
   |      unsatisfied trait bound introduced here
note: trait bound `&mut Self: ReadBytesExt` was not satisfied
  --> src/lib.rs:15:9
   |
15 | impl<R: ReadBytesExt> BinaryReader for R {}
   |         ^^^^^^^^^^^^  ------------     -
   |         |
   |         unsatisfied trait bound introduced here
help: consider relaxing the type parameter's implicit `Sized` bound
   |
15 | impl<R: ?Sized + ReadBytesExt> BinaryReader for R {}
   |         ++++++++
help: consider relaxing the type parameter's implicit `Sized` bound
   |
15 | impl<R: ?Sized + ReadBytesExt> BinaryReader for R {}
   |         ++++++++
