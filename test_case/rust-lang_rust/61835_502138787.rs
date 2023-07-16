plain
travis_time:end:07f12704:start=1560522667101281825,finish=1560522669993611222,duration=2892329397
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:09:16]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:15:15]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:15]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:15:39]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:15:48] error[E0599]: no variant or associated item named `Fn` found for type `transform::qualify_consts::Mode` in the current scope
[00:15:48]     --> src/librustc_mir/transform/qualify_consts.rs:1230:51
[00:15:48] 37   | enum Mode {
[00:15:48] 37   | enum Mode {
[00:15:48]      | --------- variant or associated item `Fn` not found here
[00:15:48] 1230 |                             if self.mode != Mode::Fn {
[00:15:48] 1230 |                             if self.mode != Mode::Fn {
[00:15:48]      |                                                   ^^ variant or associated item not found in `transform::qualify_consts::Mode`
[00:15:53] error: aborting due to previous error
[00:15:53] 
[00:15:53] For more information about this error, try `rustc --explain E0599`.
[00:15:53] error: Could not compile `rustc_mir`.
---
travis_time:end:2b05fc57:start=1560523783960527952,finish=1560523783965114945,duration=4586993
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1e23eb2d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d73f858
travis_time:start:0d73f858
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13a21f46
$ dmesg | grep -i kill
