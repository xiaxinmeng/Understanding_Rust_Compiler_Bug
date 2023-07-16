plain
travis_time:end:098c13f0:start=1553184710262449975,finish=1553184825589749695,duration=115327299720
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:24:58]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:25:39]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:25:39]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:25:39]    Compiling rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:25:40] error[E0382]: use of moved value: `sopts`
[00:25:40]    --> src/librustc_interface/util.rs:111:8
[00:25:40] 85  |     sopts: config::Options,
[00:25:40] 85  |     sopts: config::Options,
[00:25:40]     |     ----- move occurs because `sopts` has type `rustc::session::config::Options`, which does not implement the `Copy` trait
[00:25:40] 100 |         sopts,
[00:25:40]     |         ----- value moved here
[00:25:40] ...
[00:25:40] ...
[00:25:40] 111 |     if sopts.debugging_opts.unstable_options {
[00:25:40]     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ value used here after move
[00:25:41] error: aborting due to previous error
[00:25:41] 
[00:25:41] For more information about this error, try `rustc --explain E0382`.
[00:25:41] error: Could not compile `rustc_interface`.
---
travis_time:end:2a402450:start=1553186483362854563,finish=1553186483368155486,duration=5300923
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:012f9b6c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01836120
travis_time:start:01836120
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19039f59
$ dmesg | grep -i kill
