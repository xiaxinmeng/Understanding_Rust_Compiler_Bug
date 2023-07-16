
Testing libstd stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:21]    Compiling rand v0.0.0 (file:///checkout/src/librand)
[01:00:21]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:00:21]    Compiling std_unicode v0.0.0 (file:///checkout/src/libstd_unicode)
[01:00:21]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:00:27]    Compiling collections v0.0.0 (file:///checkout/src/libcollections)
[01:00:27]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[01:00:33] error[E0277]: the trait bound `u64: core::convert::TryFrom<isize>` is not satisfied
[01:00:33]    --> /checkout/src/libcore/../libcore/tests/num/mod.rs:349:24
[01:00:33]     |
[01:00:33] 349 |             assert_eq!(<$target as TryFrom<$source>>::try_from(max).unwrap(),
[01:00:33]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `core::convert::TryFrom<isize>` is not implemented for `u64`
[01:00:33] ...
[01:00:33] 383 |     test_impl_try_from_signed_to_unsigned_upper_ok! { test_try_isizeu64, isize, u64 }
[01:00:33]     |     --------------------------------------------------------------------------------- in this macro invocation
[01:00:33]     |
[01:00:33]     = help: the following implementations were found:
[01:00:33]               <u64 as core::convert::TryFrom<u16>>
[01:00:33]               <u64 as core::convert::TryFrom<i128>>
[01:00:33]               <u64 as core::convert::TryFrom<u64>>
[01:00:33]               <u64 as core::convert::TryFrom<usize>>
[01:00:33]             and 7 others
