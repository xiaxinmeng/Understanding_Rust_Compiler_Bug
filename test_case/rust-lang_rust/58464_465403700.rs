plain
travis_time:end:0c4d6e38:start=1550627163113357485,finish=1550627251407446873,duration=88294089388
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
[01:08:41] 
[01:08:41] running 119 tests
[01:09:05] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:09:09] i......iii.i.....ii
[01:09:09] 
[01:09:09]  finished in 28.295
[01:09:09] travis_fold:end:test_debuginfo

---
[01:24:10] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:10]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:24:14] error[E0658]: use of unstable library feature 'print_internals': implementation detail which may disappear or be replaced at any time
[01:24:14]    --> src/libstd/io/mod.rs:284:23
[01:24:14]     |
[01:24:14] 284 | pub use self::stdio::{_print, _eprint};
[01:24:14]     |
[01:24:14]     = help: add #![feature(print_internals)] to the crate attributes to enable
[01:24:14] 
[01:24:14] 
[01:24:14] error[E0658]: use of unstable library feature 'print_internals': implementation detail which may disappear or be replaced at any time
[01:24:14]    --> src/libstd/io/mod.rs:284:31
[01:24:14]     |
[01:24:14] 284 | pub use self::stdio::{_print, _eprint};
[01:24:14]     |
[01:24:14]     = help: add #![feature(print_internals)] to the crate attributes to enable
[01:24:14] 
[01:24:14] 
[01:24:14] error[E0658]: use of unstable library feature 'print_internals': implementation detail which may disappear or be replaced at any time
[01:24:14]     |
[01:24:14]     |
[01:24:14] 764 | pub use realstd::io::{_eprint, _print};
[01:24:14]     |
[01:24:14]     = help: add #![feature(print_internals)] to the crate attributes to enable
[01:24:14] 
[01:24:14] 
[01:24:14] error[E0658]: use of unstable library feature 'print_internals': implementation detail which may disappear or be replaced at any time
[01:24:14]     |
[01:24:14]     |
[01:24:14] 764 | pub use realstd::io::{_eprint, _print};
[01:24:14]     |
[01:24:14]     = help: add #![feature(print_internals)] to the crate attributes to enable
[01:24:14] 
[01:24:22] error: aborting due to 4 previous errors
[01:24:22] error: aborting due to 4 previous errors
[01:24:22] 
[01:24:22] For more information about this error, try `rustc --explain E0658`.
[01:24:22] error: Could not compile `std`.
[01:24:22] 
[01:24:22] To learn more, run the command again with --verbose.
[01:24:22] 
[01:24:22] 
[01:24:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:24:22] 
[01:24:22] 
[01:24:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:22] Build completed unsuccessfully in 0:26:53
[01:24:22] Build completed unsuccessfully in 0:26:53
[01:24:22] make: *** [check] Error 1
[01:24:22] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01bdb256
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 03:12:01 UTC 2019
