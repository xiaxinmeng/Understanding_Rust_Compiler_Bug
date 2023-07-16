plain
travis_time:end:05e8f635:start=1548968650482447795,finish=1548968652840920762,duration=2358472967
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:25:20]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:26:03] error: unused variable: `crate_name`
[00:26:03]    --> src/librustc_driver/lib.rs:259:25
[00:26:03]     |
[00:26:03] 259 |                     let crate_name = compiler.crate_name()?.peek().clone();
[00:26:03]     |                         ^^^^^^^^^^ help: consider using `_crate_name` instead
[00:26:03]     = note: `-D unused-variables` implied by `-D warnings`
[00:26:03] 
[00:26:03] error: aborting due to previous error
[00:26:03] 
[00:26:03] 
[00:26:03] error: Could not compile `rustc_driver`.
[00:26:03] 
[00:26:03] To learn more, run the command again with --verbose.
[00:26:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:26:03] expected success, got: exit code: 101
[00:26:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:03] Build completed unsuccessfully in 0:21:55
[00:26:03] Makefile:18: recipe for target 'all' failed
[00:26:03] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1021191c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 31 21:30:26 UTC 2019
