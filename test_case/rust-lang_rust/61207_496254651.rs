plain
travis_time:end:02e4fdde:start=1558972063508831994,finish=1558972150928208358,duration=87419376364
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:55]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:06] error[E0392]: parameter `F` is never used
[00:08:06]     --> src/librustc/middle/resolve_lifetime.rs:2122:36
[00:08:06]      |
[00:08:06] 2122 |             struct SelfVisitor<'a, F: FnMut(Res) -> bool> {
[00:08:06]      |                                    ^ unused type parameter
[00:08:06]      = help: consider removing `F` or using a marker such as `std::marker::PhantomData`
[00:08:06] 
[00:08:08] error: aborting due to previous error
[00:08:08] 
---
travis_time:end:10f91ec4:start=1558972661728973986,finish=1558972661733999277,duration=5025291
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a21a60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:021e08d7
travis_time:start:021e08d7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a73b77a
$ dmesg | grep -i kill
