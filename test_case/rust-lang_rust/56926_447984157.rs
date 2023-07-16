plain
travis_time:end:26ecbb81:start=1545077669424990271,finish=1545077670976670067,duration=1551679796
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:55]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:03:00]    Compiling compiler_builtins v0.1.2
[00:03:00]    Compiling cmake v0.1.33
[00:03:00]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:03:02] error[E0658]: the target feature `avx512f` is currently unstable (see issue #44839)
[00:03:02]   --> src/libcore/../stdsimd/coresimd/x86/avx512f.rs:13:18
[00:03:02]    |
[00:03:02] 13 | #[target_feature(enable = "avx512f")]
[00:03:02]    |
[00:03:02]    = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[00:03:02] 
[00:03:02] 
[00:03:02] error[E0658]: the target feature `avx512f` is currently unstable (see issue #44839)
[00:03:02]   --> src/libcore/../stdsimd/coresimd/x86/avx512f.rs:29:18
[00:03:02]    |
[00:03:02] 29 | #[target_feature(enable = "avx512f")]
[00:03:02]    |
[00:03:02]    = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[00:03:02] 
[00:03:02] 
[00:03:02] error[E0658]: the target feature `avx512f` is currently unstable (see issue #44839)
[00:03:02]   --> src/libcore/../stdsimd/coresimd/x86/avx512f.rs:42:18
[00:03:02]    |
[00:03:02] 42 | #[target_feature(enable = "avx512f")]
[00:03:02]    |
[00:03:02]    = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[00:03:02] 
[00:03:02] 
[00:03:02] error[E0658]: the target feature `avx512f` is currently unstable (see issue #44839)
[00:03:02]   --> src/libcore/../stdsimd/coresimd/x86/avx512f.rs:54:18
[00:03:02]    |
[00:03:02] 54 | #[target_feature(enable = "avx512f")]
[00:03:02]    |
[00:03:02]    = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[00:03:02] 
[00:03:02] 
[00:03:02] error[E0658]: the target feature `avx512f` is currently unstable (see issue #44839)
[00:03:02]   --> src/libcore/../stdsimd/coresimd/x86/avx512f.rs:63:18
[00:03:02]    |
[00:03:02] 63 | #[target_feature(enable = "avx512f")]
[00:03:02]    |
[00:03:02]    = help: add #![feature(avx512_target_feature)] to the crate attributes to enable
[00:03:02] 
[00:03:03]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
---
187088 ./obj/build/cache/2018-12-09
160388 ./obj/build/bootstrap/debug/incremental
153272 ./src/tools/clang
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7omxe8vkw-6ev22y-2sxk29axi4qzn
111160 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
95100 ./src/tools/clang/test
89968 ./src/llvm-emscripten/test/CodeGen
