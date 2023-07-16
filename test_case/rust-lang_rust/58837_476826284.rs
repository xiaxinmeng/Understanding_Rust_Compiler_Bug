plain
travis_time:end:253d6baa:start=1553623816842030562,finish=1553623817762337277,duration=920306715
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:07:14]     Checking rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:07:14] error[E0432]: unresolved import `rayon`
[00:07:14]    --> src/librustc_interface/util.rs:196:9
[00:07:14]     |
[00:07:14] 196 |     use rayon::{ThreadPool, ThreadPoolBuilder};
[00:07:14]     |         ^^^^^ use of undeclared type or module `rayon`
[00:07:15] error: aborting due to previous error
[00:07:15] 
[00:07:15] For more information about this error, try `rustc --explain E0432`.
[00:07:15] error: Could not compile `rustc_interface`.
---
travis_time:end:0d79e0d0:start=1553624267994253521,finish=1553624267999190699,duration=4937178
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bab073e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08e03f32
travis_time:start:08e03f32
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0392d578
$ dmesg | grep -i kill
