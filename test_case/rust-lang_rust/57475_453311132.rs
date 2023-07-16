plain
travis_time:end:0fe5c89c:start=1547161308643622142,finish=1547161310979617799,duration=2335995657
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |███████████████████████████████▍| 532kB 84.1MB/s eta 0:00:01
    99% |████████████████████████████████| 542kB 83.9MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 29.6MB/s 
Collecting botocore==1.12.77 (from awscli)
  Downloading https://files.pythonhosted.org/packages/f3/cd/f58bacbfb28d92716bfb7af2a8c5d5230663335b6a5e4d78d8ccdfab469d/botocore-1.12.77-py2.py3-none-any.whl (5.2MB)
    0% |▏                               | 20kB 29.5MB/s eta 0:00:01
    0% |▏                               | 30kB 35.6MB/s eta 0:00:01
    0% |▎                               | 40kB 40.3MB/s eta 0:00:01
    0% |▎                               | 51kB 43.3MB/s eta 0:00:01
---
[00:04:02] 71  |                           Some(unsafe { $Ty(n) })
[00:04:02]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:02] ...
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:02] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:02] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:02] ...   |
[00:04:02] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:02]     | |_- in this macro invocation
[00:04:02]     |
[00:04:02]     = note: #[warn(unused_unsafe)] on by default
[00:04:02] 
[00:04:02] 
[00:04:02] warning: unnecessary `unsafe` block
[00:04:02]    --> src/libcore/num/mod.rs:71:30
[00:04:02]     |
[00:04:02] 71  |                           Some(unsafe { $Ty(n) })
[00:04:02]     |                                ^^^^^^ unnecessary `unsafe` block
[00:04:02] ...
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:02] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:02] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:02] ...   |
[00:04:02] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:02]     | |_- in this macro invocation
[00:04:02] 
[00:04:02] warning: unnecessary `unsafe` block
[00:04:02]     --> src/libcore/ptr.rs:2785:18
---
[00:04:02] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:02]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:02] ...
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:02] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:02] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:02] ...   |
[00:04:02] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:02]     | |_- in this macro invocation
[00:04:02]     |
[00:04:02]     = note: #[warn(unused_attributes)] on by default
[00:04:02] 
[00:04:02] 
[00:04:02] warning: unused attribute
[00:04:02]    --> src/libcore/num/mod.rs:50:17
[00:04:02]     |
[00:04:02] 50  |                   #[rustc_layout_scalar_valid_range_start(1)]
[00:04:02]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:02] ...
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:02] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:02] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
[00:04:02] ...   |
[00:04:02] 112 | |     #[stable(feature = "signed_nonzero", since = "1.33.0")] NonZeroIsize(isize);
[00:04:02]     | |_- in this macro invocation
[00:04:02] 
[00:04:02] warning: unused attribute
[00:04:02]     --> src/libcore/ptr.rs:2720:1
---
[01:03:37] .................................................................................................... 4600/5298
[01:03:40] .................................................................................................... 4700/5298
[01:03:43] .................................................................................................... 4800/5298
[01:03:47] .................................................................................................... 4900/5298
[01:03:51] ..............................................F..................................................... 5000/5298
[01:03:58] .................................................................................................... 5200/5298
[01:04:01] .....................................i............................................................
[01:04:01] failures:
[01:04:01] 
[01:04:01] 
[01:04:01] ---- [ui] ui/try-block/try-block-bad-type.rs stdout ----
[01:04:01] diff of stderr:
[01:04:01] 
[01:04:01] 5    |         ^^^^^^^^ the trait `std::convert::From<&str>` is not implemented for `i32`
[01:04:01] 7    = help: the following implementations were found:
[01:04:01] 7    = help: the following implementations were found:
[01:04:01] -              <i32 as std::convert::From<bool>>
[01:04:01] +              <i32 as std::convert::From<core::num::NonZeroI32>>
[01:04:01] 9              <i32 as std::convert::From<i16>>
[01:04:01] -              <i32 as std::convert::From<u16>>
[01:04:01] +              <i32 as std::convert::From<i8>>
[01:04:01] 11              <i32 as std::convert::From<u8>>
[01:04:01] 13    = note: required by `std::convert::From::from`
[01:04:01] 
[01:04:01] 
[01:04:01] The actual stderr differed from the expected stderr.
[01:04:01] The actual stderr differed from the expected stderr.
[01:04:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
[01:04:01] To update references, rerun the tests and pass the `--bless` flag
[01:04:01] To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
[01:04:01] error: 1 errors occurred comparing output.
[01:04:01] status: exit code: 1
[01:04:01] status: exit code: 1
[01:04:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary" "-A" "unused"
[01:04:01] ------------------------------------------
[01:04:01] 
[01:04:01] ------------------------------------------
[01:04:01] stderr:
[01:04:01] stderr:
[01:04:01] ------------------------------------------
[01:04:01] {"message":"the trait bound `i32: std::convert::From<&str>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n