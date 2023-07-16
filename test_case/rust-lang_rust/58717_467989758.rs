plain
travis_time:end:00185549:start=1551289234908168486,finish=1551289321843955148,duration=86935786662
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
[01:14:09] 
[01:14:09] running 119 tests
[01:14:34] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:38] i......iii.i.....ii
[01:14:38] 
[01:14:38]  finished in 28.896
[01:14:38] travis_fold:end:test_debuginfo

---
[01:24:05]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:24:07] error[E0433]: failed to resolve: use of undeclared type or module `NonZeroU8`
[01:24:07]    --> src/libcore/../libcore/tests/nonzero.rs:132:47
[01:24:07]     |
[01:24:07] 132 |     assert_eq!("123".parse::<NonZeroU8>(), Ok(NonZeroU8::new(123).unwrap()));
[01:24:07]     |                                               ^^^^^^^^^ use of undeclared type or module `NonZeroU8`
[01:24:07] error[E0433]: failed to resolve: use of undeclared type or module `IntErrorKind`
[01:24:07]    --> src/libcore/../libcore/tests/nonzero.rs:136:19
[01:24:07]     |
[01:24:07] 136 |             kind: IntErrorKind::Zero
---
[01:24:07]     |
[01:24:07] 148 |             kind: IntErrorKind::Overflow
[01:24:07]     |                   ^^^^^^^^^^^^ use of undeclared type or module `IntErrorKind`
[01:24:07] 
[01:24:07] error[E0412]: cannot find type `NonZeroU8` in this scope
[01:24:07]    --> src/libcore/../libcore/tests/nonzero.rs:132:30
[01:24:07]     |
[01:24:07] 132 |     assert_eq!("123".parse::<NonZeroU8>(), Ok(NonZeroU8::new(123).unwrap()));
[01:24:07] help: a struct with a similar name exists
[01:24:07]     |
[01:24:07]     |
[01:24:07] 132 |     assert_eq!("123".parse::<NonZeroU32>(), Ok(NonZeroU8::new(123).unwrap()));
[01:24:07] help: possible candidates are found in other modules, you can import them into scope
[01:24:07]     |
[01:24:07] 1   | use core::num::NonZeroU8;
[01:24:07]     |
[01:24:07]     |
[01:24:07] 1   | use std::num::NonZeroU8;
[01:24:07]     |
[01:24:07] 
[01:24:07] error[E0412]: cannot find type `NonZeroU8` in this scope
[01:24:07]    --> src/libcore/../libcore/tests/nonzero.rs:134:21
[01:24:07]     |
[01:24:07] 134 |         "0".parse::<NonZeroU8>(),
[01:24:07] help: a struct with a similar name exists
[01:24:07]     |
[01:24:07]     |
[01:24:07] 134 |         "0".parse::<NonZeroU32>(),
[01:24:07] help: possible candidates are found in other modules, you can import them into scope
[01:24:07]     |
[01:24:07] 1   | use core::num::NonZeroU8;
[01:24:07]     |
---
[01:24:07] help: possible candidates are found in other modules, you can import them into scope
[01:24:07]     |
[01:24:07] 1   | use core::num::ParseIntError;
[01:24:07]     |
[01:24:07] 1   | use std::num::ParseIntError;
[01:24:07] 
[01:24:07] 
[01:24:07] error[E0412]: cannot find type `NonZeroU8` in this scope
[01:24:07]    --> src/libcore/../libcore/tests/nonzero.rs:140:22
[01:24:07]     |
[01:24:07] 140 |         "-1".parse::<NonZeroU8>(),
[01:24:07] help: a struct with a similar name exists
[01:24:07]     |
[01:24:07]     |
[01:24:07] 140 |         "-1".parse::<NonZeroU32>(),
[01:24:07] help: possible candidates are found in other modules, you can import them into scope
[01:24:07]     |
[01:24:07] 1   | use core::num::NonZeroU8;
[01:24:07]     |
---
[01:24:07] help: possible candidates are found in other modules, you can import them into scope
[01:24:07]     |
[01:24:07] 1   | use core::num::ParseIntError;
[01:24:07]     |
[01:24:07] 1   | use std::num::ParseIntError;
[01:24:07] 
[01:24:07] 
[01:24:07] error[E0412]: cannot find type `NonZeroU8` in this scope
[01:24:07]    --> src/libcore/../libcore/tests/nonzero.rs:146:23
[01:24:07]     |
[01:24:07] 146 |         "129".parse::<NonZeroU8>(),
[01:24:07] help: a struct with a similar name exists
[01:24:07]     |
[01:24:07]     |
[01:24:07] 146 |         "129".parse::<NonZeroU32>(),
[01:24:07] help: possible candidates are found in other modules, you can import them into scope
[01:24:07]     |
[01:24:07] 1   | use core::num::NonZeroU8;
[01:24:07]     |
---
[01:24:07] help: possible candidates are found in other modules, you can import them into scope
[01:24:07]     |
[01:24:07] 1   | use core::num::ParseIntError;
[01:24:07]     |
[01:24:07] 1   | use std::num::ParseIntError;
[01:24:07] 
[01:24:18] error: aborting due to 11 previous errors
[01:24:18] 
[01:24:18] Some errors occurred: E0412, E0422, E0433.
[01:24:18] Some errors occurred: E0412, E0422, E0433.
[01:24:18] For more information about an error, try `rustc --explain E0412`.
[01:24:18] error: Could not compile `core`.
[01:24:18] 
[01:24:18] To learn more, run the command again with --verbose.
[01:24:18] 
[01:24:18] 
[01:24:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:24:18] 
[01:24:18] 
[01:24:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:18] Build completed unsuccessfully in 0:21:43
[01:24:18] Build completed unsuccessfully in 0:21:43
[01:24:18] Makefile:48: recipe for target 'check' failed
[01:24:18] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2058aab0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 27 19:06:29 UTC 2019
---
travis_time:end:052843e4:start=1551294391340574520,finish=1551294391346558537,duration=5984017
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:061bb880
$ ln -s . checkout && for CORE i
