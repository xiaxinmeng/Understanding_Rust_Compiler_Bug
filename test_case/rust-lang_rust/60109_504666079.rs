plain
travis_time:end:2c068ed0:start=1561209845054816041,finish=1561209935661276177,duration=90606460136
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:13]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:04:14]    Compiling autocfg v0.1.4
[00:04:14]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:04:16]    Compiling backtrace v0.3.29
[00:04:19] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:38:18
[00:04:19]    |
[00:04:19] 38 | #[target_feature(enable = "sse4a")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:55:18
[00:04:19]    |
[00:04:19] 55 | #[target_feature(enable = "sse4a")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:66:18
[00:04:19]    |
[00:04:19] 66 | #[target_feature(enable = "sse4a")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `sse4a` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/sse4a.rs:77:18
[00:04:19]    |
[00:04:19] 77 | #[target_feature(enable = "sse4a")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(sse4a_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:71:18
[00:04:19]    |
[00:04:19] 71 | #[target_feature(enable = "tbm")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:82:18
[00:04:19]    |
[00:04:19] 82 | #[target_feature(enable = "tbm")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:94:18
[00:04:19]    |
[00:04:19] 94 | #[target_feature(enable = "tbm")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:105:18
[00:04:19]     |
[00:04:19] 105 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:117:18
[00:04:19]     |
[00:04:19] 117 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:128:18
[00:04:19]     |
[00:04:19] 128 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:141:18
[00:04:19]     |
[00:04:19] 141 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:153:18
[00:04:19]     |
[00:04:19] 153 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:165:18
[00:04:19]     |
[00:04:19] 165 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:176:18
[00:04:19]     |
[00:04:19] 176 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:188:18
[00:04:19]     |
[00:04:19] 188 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:199:18
[00:04:19]     |
[00:04:19] 199 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:211:18
[00:04:19]     |
[00:04:19] 211 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:222:18
[00:04:19]     |
[00:04:19] 222 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:235:18
[00:04:19]     |
[00:04:19] 235 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:247:18
[00:04:19]     |
[00:04:19] 247 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:260:18
[00:04:19]     |
[00:04:19] 260 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `tbm` is currently unstable
[00:04:19]    --> src/libcore/../stdsimd/crates/core_arch/src/x86/tbm.rs:272:18
[00:04:19]     |
[00:04:19] 272 | #[target_feature(enable = "tbm")]
[00:04:19]     |
[00:04:19]     = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]     = help: add #![feature(tbm_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `adx` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86/adx.rs:30:18
[00:04:19]    |
[00:04:19] 30 | #[target_feature(enable = "adx")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(adx_target_feature)] to the crate attributes to enable
[00:04:19] 
[00:04:19] 
[00:04:19] error[E0658]: the target feature `adx` is currently unstable
[00:04:19]   --> src/libcore/../stdsimd/crates/core_arch/src/x86_64/adx.rs:30:18
[00:04:19]    |
[00:04:19] 30 | #[target_feature(enable = "adx")]
[00:04:19]    |
[00:04:19]    = note: for more information, see https://github.com/rust-lang/rust/issues/44839
[00:04:19]    = help: add #![feature(adx_target_feature)] to the crate attributes to enable
[00:04:19] 
