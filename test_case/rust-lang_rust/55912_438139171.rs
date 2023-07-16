plain
travis_time:end:03883d91:start=1542085501697607934,finish=1542085564269252714,duration=62571644780
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:37]    Compiling log_settings v0.1.2
[00:05:37]    Compiling crossbeam-deque v0.2.0
[00:05:37]    Compiling flate2 v1.0.3
[00:05:37]    Compiling backtrace v0.3.9
[00:05:42] error[E0609]: no field `exe_allocation_crate` on type `spec::TargetOptions`
[00:05:42]   --> librustc_target/spec/powerpc_unknown_linux_musl.rs:19:10
[00:05:42]    |
[00:05:42] 19 |     base.exe_allocation_crate = None;
[00:05:42]    |
[00:05:42]    |
[00:05:42]    = note: available fields are: `is_builtin`, `linker`, `lld_flavor`, `pre_link_args`, `pre_link_args_crt` ... and 68 others
[00:05:42] 
[00:05:42] error[E0609]: no field `exe_allocation_crate` on type `spec::TargetOptions`
[00:05:42]   --> librustc_target/spec/powerpc64_unknown_linux_musl.rs:20:10
[00:05:42]    |
[00:05:42] 20 |     base.exe_allocation_crate = None;
[00:05:42]    |
[00:05:42]    |
[00:05:42]    = note: available fields are: `is_builtin`, `linker`, `lld_flavor`, `pre_link_args`, `pre_link_args_crt` ... and 68 others
[00:05:42] error: aborting due to 2 previous errors
[00:05:42] 
[00:05:42] For more information about this error, try `rustc --explain E0609`.
[00:05:42] error: Could not compile `rustc_target`.
[00:05:42] error: Could not compile `rustc_target`.
[00:05:42] warning: build failed, waiting for other jobs to finish...
[00:05:43] error: build failed
[00:05:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:43] expected success, got: exit code: 101
[00:05:43] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:05:43] travis_fold:end:stage0-rustc

[00:05:43] travis_time:end:stage0-rustc:start=1542085879061330514,finish=1542085917312139378,duration=38250808864


[00:05:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:43] Build completed unsuccessfully in 0:01:44
[00:05:43] make: *** [all] Error 1
[00:05:43] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05a18f5c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 13 05:11:57 UTC 2018
