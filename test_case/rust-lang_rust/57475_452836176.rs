plain
travis_time:end:1ab20504:start=1547059796556912748,finish=1547059797474349550,duration=917436802
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:38] 71  |                           Some(unsafe { $Ty(n) })
[00:04:38]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:38] ...
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:38] ...   |
[00:04:38] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:38]     | |_- in this macro invocation
[00:04:38]     |
[00:04:38]     = note: #[warn(unused_unsafe)] on by default
[00:04:38] 
[00:04:38] 
[00:04:38] warning: unnecessary `unsafe` block
[00:04:38]    --> src/libcore/num/mod.rs:71:30
[00:04:38]     |
[00:04:38] 71  |                           Some(unsafe { $Ty(n) })
[00:04:38]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:38] ...
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:38] ...   |
[00:04:38] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:38]     | |_- in this macro invocation
[00:04:38] 
[00:04:38] warning: unnecessary `unsafe` block
[00:04:38]     --> src/libcore/ptr.rs:2785:18
---
[00:04:38] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:38]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:38] ...
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:38] ...   |
[00:04:38] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:38]     | |_- in this macro invocation
[00:04:38]     |
[00:04:38]     = note: #[warn(unused_attributes)] on by default
[00:04:38] 
[00:04:38] 
[00:04:38] warning: unused attribute
[00:04:38]    --> src/libcore/num/mod.rs:50:17
[00:04:38]     |
[00:04:38] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:38]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:38] ...
[00:04:38] 100 | / nonzero_integers! {
[00:04:38] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:38] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:38] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:38] ...   |
[00:04:38] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:38]     | |_- in this macro invocation
[00:04:38] 
[00:04:38] warning: unused attribute
[00:04:38]     --> src/libcore/ptr.rs:2720:1
---
[01:07:11] .................................................................................................... 4600/5298
[01:07:15] .................................................................................................... 4700/5298
[01:07:18] .................................................................................................... 4800/5298
[01:07:23] .................................................................................................... 4900/5298
[01:07:27] ..............................................F..................................................... 5000/5298
[01:07:33] .................................................................................................... 5200/5298
[01:07:36] .....................................i............................................................
[01:07:36] failures:
[01:07:36] 
[01:07:36] 
[01:07:36] ---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
[01:07:36] diff of stderr:
[01:07:36] 
[01:07:36] 5    |         ^^^^^^^^ the trait `std::convert::From<&str>` is not implemented for `i32`
[01:07:36] 7    = help: the following implementations were found:
[01:07:36] 7    = help: the following implementations were found:
[01:07:36] -              <i32 as std::convert::From<bool>>
[01:07:36] +              <i32 as std::convert::From<core::num::NonZeroI32>>
[01:07:36] 9              <i32 as std::convert::From<i16>>
[01:07:36] 10              <i32 as std::convert::From<i8>>
[01:07:36] -              <i32 as std::convert::From<u16>>
[01:07:36] 12              <i32 as std::convert::From<u8>>
[01:07:36] 13    = note: required by `std::convert::From::from`
[01:07:36] 14 
[01:07:36] 14 
[01:07:36] 15 error[E0271]: type mismatch resolving `<std::result::Result<i32, i32> as std::ops::Try>::Ok == &str`
[01:07:36] 
[01:07:36] The actual stderr differed from the expected stderr.
[01:07:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
[01:07:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
[01:07:36] To update references, rerun the tests and pass the `--bless` flag
[01:07:36] To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
[01:07:36] error: 1 errors occurred comparing output.
[01:07:36] status: exit code: 1
[01:07:36] status: exit code: 1
[01:07:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary" "-A" "unused"
[01:07:36] ------------------------------------------
[01:07:36] 
[01:07:36] ------------------------------------------
[01:07:36] stderr:
[01:07:36] stderr:
[01:07:36] ------------------------------------------
[01:07:36] {"message":"the trait bound `i32: std::convert::From<&str>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n