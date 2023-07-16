rust
---- num/mod.rs - num::NonZeroI16 (line 33) stdout ----
error[E0412]: cannot find type `NonZeroI16` in module `std::num`
 --> num/mod.rs:35:39
  |
5 | assert_eq!(size_of::<Option<std::num::NonZeroI16>>(), size_of::<i16>());
  |                                       ^^^^^^^^^^ did you mean `NonZeroU16`?

thread 'num/mod.rs - num::NonZeroI16 (line 33)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
