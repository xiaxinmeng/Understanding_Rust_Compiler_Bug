plain
travis_time:end:0091f87e:start=1543880077750721433,finish=1543880080799366046,duration=3048644613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:48] 
[00:55:48] running 120 tests
[00:55:51] i..ii...iii..iiii.....i...i..........i..iii.............i.....i.....ii...i..i.ii..............i...ii 100/120
[00:55:51] ..ii.i.....iiii.....
[00:55:51] 
[00:55:51]  finished in 3.465
[00:55:51] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:06] 
[00:56:06] running 118 tests
[00:56:29] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[00:56:33] ......iii.i.....ii
[00:56:33] 
[00:56:33]  finished in 27.342
[00:56:33] travis_fold:end:test_debuginfo

---
[01:17:56]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:17:57] error[E0609]: no field `name` on type `&rustc::hir::Item`
[01:17:57]    --> src/librustc_driver/test.rs:275:25
[01:17:57]     |
[01:17:57] 275 |                 if item.name.to_string() == names[idx] {
[01:17:57] 
[01:17:58] error: aborting due to previous error
[01:17:58] 
[01:17:58] For more information about this error, try `rustc --explain E0609`.
[01:17:58] For more information about this error, try `rustc --explain E0609`.
[01:17:58] error: Could not compile `rustc_driver`.
[01:17:58] 
[01:17:58] To learn more, run the command again with --verbose.
[01:17:58] 
[01:17:58] 
[01:17:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:17:58] 
[01:17:58] 
[01:17:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:58] Build completed unsuccessfully in 0:32:43
[01:17:58] Build completed unsuccessfully in 0:32:43
[01:17:58] Makefile:58: recipe for target 'check' failed
[01:17:58] make: *** [check] Error 1
