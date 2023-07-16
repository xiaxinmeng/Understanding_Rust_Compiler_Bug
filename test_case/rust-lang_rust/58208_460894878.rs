plain
travis_time:end:050047e5:start=1549425809935218963,finish=1549425812322263801,duration=2387044838
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:26]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:26]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:26]    Compiling rustc-demangle v0.1.10
[00:04:31]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:32] error: extern crate `libc` is private, and cannot be re-exported (error E0365), consider declaring with `pub`
[00:04:32]   --> src/libstd/sys/unix/net.rs:15:9
[00:04:32]    |
[00:04:32] 15 | pub use libc as netc;
[00:04:32]    |
[00:04:32]    |
[00:04:32]    = note: #[deny(pub_use_of_private_extern_crate)] on by default
[00:04:32]    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:04:32] 
2752840 .
1346352 ./src
1313156 ./obj
---
199364 ./obj/build/cache/2019-01-18
156148 ./src/llvm-project/clang
155464 ./obj/build/bootstrap/debug/incremental
140688 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e
140684 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e/s-f984glc7ym-16gm6th-1rhegn0zm30b4
108528 ./src/llvm-project/lldb
97552 ./src/llvm-project/clang/test
92608 ./.git
89964 ./src/llvm-emscripten/test/CodeGen
