plain
travis_time:end:0e84c352:start=1549073171785013704,finish=1549073245585511535,duration=73800497831
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
[01:10:06] 
[01:10:06] running 119 tests
[01:10:31] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:10:35] i......iii.i.....ii
[01:10:35] 
[01:10:35]  finished in 28.905
[01:10:35] travis_fold:end:test_debuginfo

---
[01:31:20] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:31:21]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:31:22] error[E0277]: `F` cannot be sent between threads safely
[01:31:22]    --> src/librustc_driver/test.rs:106:5
[01:31:22]     |
[01:31:22] 106 |     interface::run_compiler(config, |compiler| {
[01:31:22]     |     ^^^^^^^^^^^^^^^^^^^^^^^ `F` cannot be sent between threads safely
[01:31:22]     |
[01:31:22]     = help: within `[closure@src/librustc_driver/test.rs:106:37: 127:6 body:F, expected_err_count:&usize]`, the trait `std::marker::Send` is not implemented for `F`
[01:31:22]     = help: consider adding a `where F: std::marker::Send` bound
[01:31:22]     = note: required because it appears within the type `[closure@src/librustc_driver/test.rs:106:37: 127:6 body:F, expected_err_count:&usize]`
[01:31:22]     = note: required by `rustc_interface::run_compiler`
[01:31:22] error: aborting due to previous error
[01:31:22] 
[01:31:22] For more information about this error, try `rustc --explain E0277`.
[01:31:22] error: Could not compile `rustc_driver`.
[01:31:22] error: Could not compile `rustc_driver`.
[01:31:22] 
[01:31:22] To learn more, run the command again with --verbose.
[01:31:22] 
[01:31:22] 
[01:31:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:31:22] 
[01:31:22] 
[01:31:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:22] Build completed unsuccessfully in 0:32:49
[01:31:22] Build completed unsuccessfully in 0:32:49
[01:31:22] make: *** [check] Error 1
[01:31:22] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:217a50f6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 03:38:57 UTC 2019
---
travis_time:end:0ad695c5:start=1549078739479664647,finish=1549078739484363860,duration=4699213
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cd27961
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EX
