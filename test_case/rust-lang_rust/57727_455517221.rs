plain
[00:04:01] 71  |                           Some(unsafe { $Ty(n) })
[00:04:01]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:01] ...
[00:04:01] 100 | / nonzero_integers! {
[00:04:01] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:01] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:01] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:01] ...   |
[00:04:01] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[00:04:01]     | |_- in this macro invocation
[00:04:01]     |
[00:04:01]     = note: #[warn(unused_unsafe)] on by default
[00:04:01] 
[00:04:01] 
[00:04:01] warning: unnecessary `unsafe` block
[00:04:01]    --> src/libcore/num/mod.rs:71:30
[00:04:01]     |
[00:04:01] 71  |                           Some(unsafe { $Ty(n) })
[00:04:01]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:01] ...
[00:04:01] 100 | / nonzero_integers! {
[00:04:01] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:01] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:01] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:01] ...   |
[00:04:01] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[00:04:01]     | |_- in this macro invocation
[00:04:01] 
[00:04:01] warning: unnecessary `unsafe` block
[00:04:01]     --> src/libcore/ptr.rs:2786:18
---
[00:04:01] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:01]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:01] ...
[00:04:01] 100 | / nonzero_integers! {
[00:04:01] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:01] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:01] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:01] ...   |
[00:04:01] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[00:04:01]     | |_- in this macro invocation
[00:04:01]     |
[00:04:01]     = note: #[warn(unused_attributes)] on by default
[00:04:01] 
[00:04:01] 
[00:04:01] warning: unused attribute
[00:04:01]    --> src/libcore/num/mod.rs:50:17
[00:04:01]     |
[00:04:01] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:01]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:01] ...
[00:04:01] 100 | / nonzero_integers! {
[00:04:01] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:01] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:01] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:01] ...   |
[00:04:01] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[00:04:01]     | |_- in this macro invocation
[00:04:01] 
[00:04:01] warning: unused attribute
[00:04:01]     --> src/libcore/ptr.rs:2721:1
---
[01:00:59] 71  |                           Some(unsafe { $Ty(n) })
[01:00:59]     |                                ^^^^^^ unnecessary `unsafe` block
[01:00:59] ...
[01:00:59] 100 | / nonzero_integers! {
[01:00:59] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[01:00:59] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[01:00:59] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[01:00:59] ...   |
[01:00:59] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[01:00:59]     | |_- in this macro invocation
[01:00:59]     |
[01:00:59]     = note: #[warn(unused_unsafe)] on by default
[01:00:59] 
[01:00:59] 
[01:00:59] warning: unnecessary `unsafe` block
[01:00:59]    --> src/libcore/num/mod.rs:71:30
[01:00:59]     |
[01:00:59] 71  |                           Some(unsafe { $Ty(n) })
[01:00:59]     |                                ^^^^^^ unnecessary `unsafe` block
[01:00:59] ...
[01:00:59] 100 | / nonzero_integers! {
[01:00:59] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[01:00:59] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[01:00:59] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[01:00:59] ...   |
[01:00:59] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[01:00:59]     | |_- in this macro invocation
[01:00:59] 
[01:00:59] warning: unnecessary `unsafe` block
[01:00:59]     --> src/libcore/ptr.rs:2786:18
---
[01:00:59] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[01:00:59]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:00:59] ...
[01:00:59] 100 | / nonzero_integers! {
[01:00:59] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[01:00:59] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[01:00:59] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[01:00:59] ...   |
[01:00:59] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[01:00:59]     | |_- in this macro invocation
[01:00:59]     |
[01:00:59]     = note: #[warn(unused_attributes)] on by default
[01:00:59] 
[01:00:59] 
[01:00:59] warning: unused attribute
[01:00:59]    --> src/libcore/num/mod.rs:50:17
[01:00:59]     |
[01:00:59] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[01:00:59]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:00:59] ...
[01:00:59] 100 | / nonzero_integers! {
[01:00:59] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[01:00:59] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[01:00:59] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[01:00:59] ...   |
[01:00:59] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[01:00:59]     | |_- in this macro invocation
[01:00:59] 
[01:00:59] warning: unused attribute
[01:00:59]     --> src/libcore/ptr.rs:2721:1
---
[01:06:20] 
[01:06:20] ---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
[01:06:20] diff of stderr:
[01:06:20] 
[01:06:20] 5    |         ^^^^^^^^ the trait `std::convert::From<&str>` is not implemented for `i32`
[01:06:20] 7    = help: the following implementations were found:
[01:06:20] 7    = help: the following implementations were found:
[01:06:20] -              <i32 as std::convert::From<core::num::NonZeroI32>>
[01:06:20] +              <i32 as std::convert::From<bool>>
[01:06:20] 9              <i32 as std::convert::From<i16>>
[01:06:20] -              <i32 as std::convert::From<i8>>
[01:06:20] +              <i32 as std::convert::From<u16>>
[01:06:20] 11              <i32 as std::convert::From<u8>>
[01:06:20] 13    = note: required by `std::convert::From::from`
[01:06:20] 
[01:06:20] 
[01:06:20] The actual stderr differed from the expected stderr.
[01:06:20] The actual stderr differed from the expected stderr.
[01:06:20] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
[01:06:20] To update references, rerun the tests and pass the `--bless` flag
[01:06:20] To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
[01:06:20] error: 1 errors occurred comparing output.
[01:06:20] status: exit code: 1
[01:06:20] status: exit code: 1
[01:06:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary" "-A" "unused"
[01:06:20] ------------------------------------------
[01:06:20] 
[01:06:20] ------------------------------------------
[01:06:20] stderr:
[01:06:20] stderr:
[01:06:20] ------------------------------------------
[01:06:20] {"message":"the trait bound `i32: std::convert::From<&str>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n