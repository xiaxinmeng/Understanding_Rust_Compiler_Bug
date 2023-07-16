plain
travis_time:end:02e96b48:start=1558614992351566021,finish=1558615085041050302,duration=92689484281
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:28:35]    Compiling rustc-demangle v0.1.10
[00:28:41]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:28:41]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:28:41]    Compiling hashbrown v0.3.0
[00:28:49] error: internal compiler error: src/librustc_typeck/check/mod.rs:830: can't type-check body of DefId(0:8405 ~ std[a258]::sys[0]::unix[0]::fast_thread_local[0]::register_dtor[0]::[0]::__cxa_thread_atexit_impl[0])
[00:28:49]   --> src/libstd/sys/unix/fast_thread_local.rs:22:9
[00:28:49]    |
[00:28:49] 22 |         static __cxa_thread_atexit_impl: *const libc::c_void;
[00:28:49] 
[00:28:49] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:572:9
[00:28:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:28:49] error: aborting due to previous error
---
[00:28:49] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:28:49] 
[00:28:49] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[00:28:49] 
[00:28:49] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C prefer-dynamic -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type dylib --crate-type rlib
[00:28:49] note: some of the compiler flags provided by cargo are hidden
[00:28:49] 
[00:28:49] error: Could not compile `std`.
[00:28:49] 
---
156496 ./src/llvm-project/clang
145240 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
145236 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
142484 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9
142480 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9/s-fchdycsilv-18hx6zb-1bayru0z1wx58
123648 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
97584 ./src/llvm-project/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
