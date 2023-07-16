plain
travis_time:end:0102a977:start=1553820976800674553,finish=1553820978953193112,duration=2152518559
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:08:26]  Documenting std v0.0.0 (/checkout/src/libstd)
[01:08:27] error[E0106]: missing lifetime specifier
[01:08:27]   --> src/libstd/sys/windows/ext/io.rs:87:22
[01:08:27]    |
[01:08:27] 87 | impl AsRawHandle for io::StdinLock {
[01:08:27] 
[01:08:27] error[E0106]: missing lifetime specifier
[01:08:27]   --> src/libstd/sys/windows/ext/io.rs:94:22
[01:08:27]    |
[01:08:27]    |
[01:08:27] 94 | impl AsRawHandle for io::StdoutLock {
[01:08:27] 
[01:08:27] error[E0106]: missing lifetime specifier
[01:08:27]    --> src/libstd/sys/windows/ext/io.rs:101:22
[01:08:27]     |
[01:08:27]     |
[01:08:27] 101 | impl AsRawHandle for io::StderrLock {
[01:08:27] 
[01:08:27] error: aborting due to 3 previous errors
[01:08:27] 
[01:08:27] For more information about this error, try `rustc --explain E0106`.
[01:08:27] For more information about this error, try `rustc --explain E0106`.
[01:08:27] error: Could not document `std`.
[01:08:27] 
[01:08:27] Caused by:
[01:08:27]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name std src/libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="backtrace-sys"' --cfg 'feature="compiler_builtins"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.35.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-e65aa2f83aa56aa2.rmeta --extern backtrace_sys=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-cede990af8bcb41f.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-e80a174c39ec913a.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-c94a89edb07c8a5f.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-10168d2d2a1f5edb.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-ddab8ec2cd11791f.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-1f62d5a828035933.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-d0e3a87aaa5e22ca.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-45d09188ef721c31.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-bd3fd5da4c9ebcde.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-2f9037ec5b18fde6.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-a638434c2e8ed7e7.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-9a686dab8da424f7.rmeta` (exit code: 1)
[01:08:27] 
[01:08:27] 
[01:08:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.35.0" "--index-page" "/checkout/src/doc/index.md"
[01:08:27] 
[01:08:27] 
[01:08:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:08:27] Build completed unsuccessfully in 0:08:15
---
travis_time:end:19f8e044:start=1553825099527281206,finish=1553825099532210161,duration=4928955
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1006248f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20038980
travis_time:start:20038980
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07884df2
$ dmesg | grep -i kill
