plain
travis_time:end:0caccb04:start=1542953857766570812,finish=1542953858830289192,duration=1063718380
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:53:16]     Checking compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:53:17]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:53:23]     Finished release [optimized] target(s) in 45.12s
[00:53:23]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:53:28] error: cannot find macro `call!` in this scope
[00:53:28]   --> src/libcore/../stdsimd/coresimd/arm/armclang.rs:58:25
[00:53:28]    |
[00:53:28] 58 |     constify_imm8!(val, call);
[00:53:28]    |
[00:53:28]    |
[00:53:28]    = help: have you added the `#[macro_use]` on the module/import?
_64-unknown-linux-gnu/release
119368 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
119364 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
118444 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
