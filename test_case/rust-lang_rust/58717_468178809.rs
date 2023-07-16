plain
travis_time:end:1dd7a369:start=1551336453136325981,finish=1551336529044450194,duration=75908124213
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:15:26] 
[01:15:26] running 119 tests
[01:15:53] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:15:58] i......iii.i.....ii
[01:15:58] 
[01:15:58]  finished in 31.575
[01:15:58] travis_fold:end:test_debuginfo

---
[01:25:11] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:11]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:25:14] error[E0658]: use of unstable library feature 'int_error_matching': it can be useful to match errors when making error messages for integer parsing (see issue #22639)
[01:25:14]  --> src/libcore/../libcore/tests/nonzero.rs:1:17
[01:25:14]   |
[01:25:14] 1 | use core::num::{IntErrorKind, NonZeroU8, NonZeroU32, NonZeroI32, ParseIntError};
[01:25:14]   |
[01:25:14]   = help: add #![feature(int_error_matching)] to the crate attributes to enable
[01:25:14] 
[01:25:14] 
[01:25:14] error[E0658]: use of unstable library feature 'int_error_matching': it can be useful to match errors when making error messages for integer parsing (see issue #22639)
[01:25:14]    --> src/libcore/../libcore/tests/nonzero.rs:135:19
[01:25:14] 135 |             kind: IntErrorKind::Zero
[01:25:14]     |                   ^^^^^^^^^^^^^^^^^^
[01:25:14]     |
[01:25:14]     = help: add #![feature(int_error_matching)] to the crate attributes to enable
[01:25:14]     = help: add #![feature(int_error_matching)] to the crate attributes to enable
[01:25:14] 
[01:25:14] error[E0658]: use of unstable library feature 'int_error_matching': it can be useful to match errors when making error messages for integer parsing (see issue #22639)
[01:25:14]    --> src/libcore/../libcore/tests/nonzero.rs:141:19
[01:25:14] 141 |             kind: IntErrorKind::Underflow
[01:25:14]     |                   ^^^^^^^^^^^^^^^^^^^^^^^
[01:25:14]     |
[01:25:14]     = help: add #![feature(int_error_matching)] to the crate attributes to enable
[01:25:14]     = help: add #![feature(int_error_matching)] to the crate attributes to enable
[01:25:14] 
[01:25:14] error[E0658]: use of unstable library feature 'int_error_matching': it can be useful to match errors when making error messages for integer parsing (see issue #22639)
[01:25:14]    --> src/libcore/../libcore/tests/nonzero.rs:147:19
[01:25:14] 147 |             kind: IntErrorKind::Overflow
[01:25:14]     |                   ^^^^^^^^^^^^^^^^^^^^^^
[01:25:14]     |
[01:25:14]     = help: add #![feature(int_error_matching)] to the crate attributes to enable
---
[01:25:24] 
[01:25:24] To learn more, run the command again with --verbose.
[01:25:24] 
[01:25:24] 
[01:25:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:25:24] 
[01:25:24] 
[01:25:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:24] Build completed unsuccessfully in 0:21:56
[01:25:24] Build completed unsuccessfully in 0:21:56
[01:25:24] Makefile:48: recipe for target 'check' failed
[01:25:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:080c14aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 28 08:14:21 UTC 2019
---
travis_time:end:0568bf28:start=1551341663882453381,finish=1551341663887550940,duration=5097559
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:020398ea
$ ln -s . checkout && for CORE i
