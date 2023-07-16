plain
travis_time:end:1c354f5a:start=1556396103388458433,finish=1556396105274031703,duration=1885573270
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:37]    Compiling rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:04:38]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:38]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:38]    Compiling rustc-demangle v0.1.10
[00:04:42] error: method is never used: `handle_cap_increase`
[00:04:42]     |
[00:04:42]     |
[00:04:42] 358 |     unsafe fn handle_cap_increase(&mut self, old_capacity: usize) {
[00:04:42]     |
[00:04:42]     = note: `-D dead-code` implied by `-D warnings`
[00:04:42] 
[00:04:42] error: aborting due to previous error
---
travis_time:end:00bc8f40:start=1556396400075287209,finish=1556396400080357821,duration=5070612
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b21ecb8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e7e9dcc
travis_time:start:0e7e9dcc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:s
