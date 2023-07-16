plain
travis_time:end:03471c00:start=1552348735192620946,finish=1552348811489474158,duration=76296853212
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:31]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:35] error[E0432]: unresolved import `task::Context`
[00:03:35]  --> src/libcore/future/future.rs:8:12
[00:03:35]   |
[00:03:35] 8 | use task::{Context, Poll};
[00:03:35]   |            ^^^^^^^ no `Context` in `task`
[00:03:37]    Compiling compiler_builtins v0.1.5
[00:03:37]    Compiling cmake v0.1.33
[00:03:37]    Compiling backtrace-sys v0.1.27
[00:03:40]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:40]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:40]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:40]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:41]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:41]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:47] error[E0308]: mismatched types
[00:03:47]    --> src/libcore/task/wake.rs:117:9
[00:03:47]     |
[00:03:47] 116 |       ) -> RawWaker {
[00:03:47]     |            -------- expected `task::wake::RawWaker` because of return type
[00:03:47] 117 | /         RawWakerVTable {
[00:03:47] 119 | |             wake,
[00:03:47] 120 | |             drop,
[00:03:47] 121 | |         }
[00:03:47] 121 | |         }
[00:03:47]     | |_________^ expected struct `task::wake::RawWaker`, found struct `task::wake::RawWakerVTable`
[00:03:47]     |
[00:03:47]     = note: expected type `task::wake::RawWaker`
[00:03:47]                found type `task::wake::RawWakerVTable`
[00:03:49] error: aborting due to 2 previous errors
[00:03:49] 
[00:03:49] Some errors occurred: E0308, E0432.
[00:03:49] For more information about an error, try `rustc --explain E0308`.
---
travis_time:end:0e2ece00:start=1552349050582176853,finish=1552349050587060466,duration=4883613
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01aad72e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f3d35a7
travis_time:start:1f3d35a7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b2028f0
$ dmesg | grep -i kill
