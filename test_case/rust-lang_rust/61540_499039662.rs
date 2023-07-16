plain
travis_time:end:0ff34e86:start=1559732482795789211,finish=1559732483673907334,duration=878118123
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:00]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:05:00]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:05:01]    Compiling autocfg v0.1.4
[00:05:03]    Compiling backtrace v0.3.25
[00:05:05] error[E0658]: the target feature `rtm` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/rtm.rs:58:18
[00:05:05]    |
[00:05:05] 58 | #[target_feature(enable = "rtm")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(rtm_target_feature)] to the crate attributes to enable
[00:05:05] 
[00:05:05] 
[00:05:05] error[E0658]: the target feature `rtm` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/rtm.rs:68:18
[00:05:05]    |
[00:05:05] 68 | #[target_feature(enable = "rtm")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(rtm_target_feature)] to the crate attributes to enable
[00:05:05] 
[00:05:05] 
[00:05:05] error[E0658]: the target feature `rtm` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/rtm.rs:78:18
[00:05:05]    |
[00:05:05] 78 | #[target_feature(enable = "rtm")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(rtm_target_feature)] to the crate attributes to enable
[00:05:05] 
[00:05:05] 
[00:05:05] error[E0658]: the target feature `rtm` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/rtm.rs:95:18
[00:05:05]    |
[00:05:05] 95 | #[target_feature(enable = "rtm")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(rtm_target_feature)] to the crate attributes to enable
[00:05:05] 
[00:05:05] 
[00:05:05] error[E0658]: the target feature `f16c` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/f16c.rs:30:18
[00:05:05]    |
[00:05:05] 30 | #[target_feature(enable = "f16c")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(f16c_target_feature)] to the crate attributes to enable
[00:05:05] 
[00:05:05] 
[00:05:05] error[E0658]: the target feature `f16c` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/f16c.rs:39:18
[00:05:05]    |
[00:05:05] 39 | #[target_feature(enable = "f16c")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(f16c_target_feature)] to the crate attributes to enable
[00:05:05] 
[00:05:05] 
[00:05:05] error[E0658]: the target feature `f16c` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/f16c.rs:73:18
[00:05:05]    |
[00:05:05] 73 | #[target_feature(enable = "f16c")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(f16c_target_feature)] to the crate attributes to enable
[00:05:05] 
[00:05:05] 
[00:05:05] error[E0658]: the target feature `f16c` is currently unstable
[00:05:05]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/f16c.rs:97:18
[00:05:05]    |
[00:05:05] 97 | #[target_feature(enable = "f16c")]
[00:05:05]    |
[00:05:05]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:05:05]    = help: add #![feature(f16c_target_feature)] to the crate attributes to enable
[00:05:05] 
---
travis_time:end:184d7bc6:start=1559732812745362274,finish=1559732812750763593,duration=5401319
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1acf921a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1453dcdc
travis_time:start:1453dcdc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:e
