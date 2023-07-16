plain
travis_time:end:0d7fd999:start=1561208490541277731,finish=1561208579327100477,duration=88785822746
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:22]    Compiling autocfg v0.1.4
[00:04:23]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:24]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:24]    Compiling backtrace v0.3.29
[00:04:27] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:38:18
[00:04:27]    |
[00:04:27] 38 | #[target_feature(enable = "sse4a")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:55:18
[00:04:27]    |
[00:04:27] 55 | #[target_feature(enable = "sse4a")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:66:18
[00:04:27]    |
[00:04:27] 66 | #[target_feature(enable = "sse4a")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:77:18
[00:04:27]    |
[00:04:27] 77 | #[target_feature(enable = "sse4a")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:71:18
[00:04:27]    |
[00:04:27] 71 | #[target_feature(enable = "tbm")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:82:18
[00:04:27]    |
[00:04:27] 82 | #[target_feature(enable = "tbm")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:94:18
[00:04:27]    |
[00:04:27] 94 | #[target_feature(enable = "tbm")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:105:18
[00:04:27]     |
[00:04:27] 105 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:117:18
[00:04:27]     |
[00:04:27] 117 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:128:18
[00:04:27]     |
[00:04:27] 128 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:141:18
[00:04:27]     |
[00:04:27] 141 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:153:18
[00:04:27]     |
[00:04:27] 153 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:165:18
[00:04:27]     |
[00:04:27] 165 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:176:18
[00:04:27]     |
[00:04:27] 176 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:188:18
[00:04:27]     |
[00:04:27] 188 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:199:18
[00:04:27]     |
[00:04:27] 199 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:211:18
[00:04:27]     |
[00:04:27] 211 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:222:18
[00:04:27]     |
[00:04:27] 222 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:235:18
[00:04:27]     |
[00:04:27] 235 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:247:18
[00:04:27]     |
[00:04:27] 247 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:260:18
[00:04:27]     |
[00:04:27] 260 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `tbm` is currently unstable
[00:04:27]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:272:18
[00:04:27]     |
[00:04:27] 272 | #[target_feature(enable = "tbm")]
[00:04:27]     |
[00:04:27]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `adx` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/adx.rs:30:18
[00:04:27]    |
[00:04:27] 30 | #[target_feature(enable = "adx")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(adx_target_feature)] to the crate attributes to enable
[00:04:27] 
[00:04:27] 
[00:04:27] error[E0658]: the target feature `adx` is currently unstable
[00:04:27]   --> src/libcore/../stdsimd/crates/core_arch/src/x86_64/adx.rs:30:18
[00:04:27]    |
[00:04:27] 30 | #[target_feature(enable = "adx")]
[00:04:27]    |
[00:04:27]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:27]    = help: add #![feature(adx_target_feature)] to the crate attributes to enable
[00:04:27] 
---
travis_time:end:0c0fb9b0:start=1561208868308528291,finish=1561208868313384650,duration=4856359
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:005a31d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22dae3a4
travis_time:start:22dae3a4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05a8aeaf
$ dmesg | grep -i kill
