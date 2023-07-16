plain
travis_time:end:1d07b310:start=1558571992361450311,finish=1558571993165491254,duration=804040943
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:59]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:06]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:10]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:14]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:14:22]    Compiling rustc_monomorphize v0.0.0 (/checkout/src/librustc_monomorphize)
[00:17:25]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:17:26]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:18:21]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:18:23]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
---
[00:27:27]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:27:27]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:27:28]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:28]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:27:53] error: internal compiler error: src/librustc/ty/query/mod.rs:101: tcx.collect_and_partition_mono_items(crate0) unsupported by its crate
[00:27:53] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[00:27:53] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:27:53] error: aborting due to previous error
[00:27:53] 
[00:27:53] 
[00:27:53] 
[00:27:53] note: the compiler unexpectedly panicked. this is a bug.
[00:27:53] 
[00:27:53] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:27:53] 
[00:27:53] note: rustc 1.36.0-dev running on x86_64-unknown-linux-gnu
[00:27:53] 
[00:27:53] note: compiler flags: -Z external-macro-backtrace -Z unstable-options -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:27:53] note: some of the compiler flags provided by cargo are hidden
[00:27:53] 
[00:27:53] error: Could not compile `core`.
[00:27:53] 
---
19656 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
19256 ./src/llvm-project/lldb/www/cpp_reference
19252 ./src/llvm-project/lldb/www/cpp_reference/html
travis_time:end:1664d2d0:start=1558573677533508712,finish=1558573678169073130,dura[0K$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00d1a2ee
$ dmesg | grep -i kill
