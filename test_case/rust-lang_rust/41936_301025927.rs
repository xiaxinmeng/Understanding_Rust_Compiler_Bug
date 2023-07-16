rust
error: main function not found

error[E0391]: unsupported cyclic reference between types/traits detected
 --> 1.rs:7:19
  |
7 | struct LogDataBuf([u8;8]);
  |                   ^^^^^^^ cyclic reference
  |
note: the cycle begins when processing `LogDataBuf::0`...
 --> 1.rs:7:19
  |
7 | struct LogDataBuf([u8;8]);
  |                   ^^^^^^^
note: ...which then requires const-evaluating `LogDataBuf::{{initializer}}`...
 --> 1.rs:7:23
  |
7 | struct LogDataBuf([u8;8]);
  |                       ^
note: ...which then requires processing `LogDataBuf::{{initializer}}`...
 --> 1.rs:7:23
  |
7 | struct LogDataBuf([u8;8]);
  |                       ^
note: ...which then requires coherence checking all impls of trait `std::ops::CoerceUnsized`...
note: ...which then requires processing `<impl at 1.rs:14:1: 14:90>`...
 --> 1.rs:14:1
  |
14| impl<T: ?Sized + marker::Unsize<U>, U: ?Sized> ops::CoerceUnsized<Aref<U>> for Aref<T> {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which then requires processing `ArefInner`...
 --> 1.rs:16:1
  |
16| / struct ArefInner<T: ?Sized>
17| | {
18| |     // Even with this field commented out, the error is raised.
19| |     data: T,
20| | }
  | |_^
note: ...which then requires computing the variances for items in this crate...
  = note: ...which then again requires processing `LogDataBuf::0`, completing the cycle.

error: aborting due to previous error
