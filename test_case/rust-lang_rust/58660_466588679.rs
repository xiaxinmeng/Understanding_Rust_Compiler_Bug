plain
travis_time:end:26681e3c:start=1550876296723837634,finish=1550876297696067905,duration=972230271
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |███████████████████████████████▍| 532kB 58.6MB/s eta 0:00:01
    99% |████████████████████████████████| 542kB 56.1MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 26.4MB/s 
Collecting botocore==1.12.101 (from awscli)
  Downloading https://files.pythonhosted.org/packages/d2/c1/bba75ff036a9363d32f597cddd0fbb6b6166a51f896631c4f57e56aacf65/botocore-1.12.101-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 29.7MB/s eta 0:00:01
    0% |▏                               | 30kB 34.7MB/s eta 0:00:01
    0% |▎                               | 40kB 38.2MB/s eta 0:00:01
    0% |▎                               | 51kB 41.5MB/s eta 0:00:01
---
[00:56:04]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:56:21] warning: `[into_initialized]` cannot be resolved, ignoring it...
[00:56:21]     --> src/libcore/mem.rs:1326:46
[00:56:21]      |
[00:56:21] 1326 |     /// calling `read_initialized` and then [`into_initialized`]), it is your responsibility
[00:56:21]      |
[00:56:21] note: lint level defined here
[00:56:21]     --> src/libcore/lib.rs:63:9
[00:56:21]      |
---
[00:56:33]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:56:33]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:56:33]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:56:33]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:56:40] error: `[into_initialized]` cannot be resolved, ignoring it...
[00:56:40]      |
[00:56:40]      |
[00:56:40] 1326 |     /// calling `read_initialized` and then [`into_initialized`]), it is your responsibility
[00:56:40]      |
[00:56:40] note: lint level defined here
[00:56:40]     --> src/libstd/lib.rs:209:9
[00:56:40]      |
[00:56:40]      |
[00:56:40] 209  | #![deny(intra_doc_link_resolution_failure)]
[00:56:40]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:56:40]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:40] 
[00:56:40] error: Could not document `std`.
[00:56:40] 
[00:56:40] Caused by:
[00:56:40]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std src/libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="backtrace-sys"' --cfg 'feature="compiler_builtins"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-9e6c0311b71511c6.rmeta --extern backtrace_sys=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-f13b165ed9b4dd57.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d54fe968dea87029.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-84e5b9599b1b7754.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-a69fda92b07aedd5.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-349a3e5cce9f18ee.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-d7c48504ff1056b6.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-20814bca47e9a554.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-8bc4c4bf1d4ec8f0.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0814373332ff833c.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-655667bc8a1522d0.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-5756fbda854daecf.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-5e31c590860d0940.rmeta` (exit code: 1)
[00:56:40] 
[00:56:40] 
[00:56:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--index-page" "/checkout/src/doc/index.md"
[00:56:40] 
[00:56:40] 
[00:56:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:56:40] Build completed unsuccessfully in 0:06:07
---
travis_time:end:05402bb0:start=1550879710959348000,finish=1550879710965134673,duration=5786673
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:100dca54
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bcc43f4
travis_time:start:0bcc43f4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:031f8dfe
$ dmesg | grep -i kill
