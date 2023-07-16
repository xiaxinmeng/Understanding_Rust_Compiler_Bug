plain
travis_time:end:05295aec:start=1559388157038476135,finish=1559388157793581933,duration=755105798
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:26:29]    Compiling rustc-demangle v0.1.15
[00:26:33]    Compiling memmap v0.6.2
[00:26:34]    Compiling num_cpus v1.8.0
[00:26:38]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:26:42] error: use of deprecated item 'libc::uint32_t': Use u32 instead.
[00:26:42]    --> src/librustc_codegen_llvm/llvm/ffi.rs:569:29
[00:26:42] 569 |         pub struct DIFlags: ::libc::uint32_t {
[00:26:42]     |                             ^^^^^^^^^^^^^^^^
[00:26:42]     |
[00:26:42]     = note: `-D deprecated` implied by `-D warnings`
[00:26:42]     = note: `-D deprecated` implied by `-D warnings`
[00:26:42] 
[00:26:42] error: use of deprecated item 'libc::uint32_t': Use u32 instead.
[00:26:42]    --> src/librustc_codegen_llvm/llvm/ffi.rs:598:31
[00:26:42]     |
[00:26:42] 598 |         pub struct DISPFlags: ::libc::uint32_t {
[00:26:42] 
[00:26:42] 
[00:26:42] error: use of deprecated item 'libc::uint32_t': Use u32 instead.
[00:26:42]    --> src/librustc_codegen_llvm/llvm/ffi.rs:569:29
[00:26:42] 569 |         pub struct DIFlags: ::libc::uint32_t {
[00:26:42]     |                             ^^^^^^^^^^^^^^^^
[00:26:42] 
[00:26:44] error: aborting due to 3 previous errors
---
19256 ./src/llvm-project/lldb/www/cpp_reference/html
18956 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build
17940 ./src/llvm-project/lldb/www/python_reference
travis_time:end:0702be1a:start=1559389773435770534,finish=1559389774041021194b8b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06ef8afe
$ dmesg | grep -i kill
