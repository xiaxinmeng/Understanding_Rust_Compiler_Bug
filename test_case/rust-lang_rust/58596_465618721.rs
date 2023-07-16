plain
travis_time:end:1dd7be70:start=1550672011582408772,finish=1550672012398773168,duration=816364396
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:58:26]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:58:42] warning: `[char::escape_debug]` cannot be resolved, ignoring it...
[00:58:42]     --> src/libcore/str/mod.rs:3968:67
[00:58:42]      |
[00:58:42] 3968 |     /// Return an iterator that escapes each char in `self` with [`char::escape_debug`].
[00:58:42]      |
[00:58:42] note: lint level defined here
[00:58:42]     --> src/libcore/lib.rs:63:9
[00:58:42]      |
[00:58:42]      |
[00:58:42] 63   | #![warn(intra_doc_link_resolution_failure)]
[00:58:42]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:58:42]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:58:42] 
[00:58:42] warning: `[char::escape_default]` cannot be resolved, ignoring it...
[00:58:42]     --> src/libcore/str/mod.rs:4014:67
[00:58:42]      |
[00:58:42] 4014 |     /// Return an iterator that escapes each char in `self` with [`char::escape_default`].
[00:58:42]      |
[00:58:42]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:58:42] 
[00:58:42] warning: `[char::escape_unicode]` cannot be resolved, ignoring it...
[00:58:42] warning: `[char::escape_unicode]` cannot be resolved, ignoring it...
[00:58:42]     --> src/libcore/str/mod.rs:4050:67
[00:58:42]      |
[00:58:42] 4050 |     /// Return an iterator that escapes each char in `self` with [`char::escape_unicode`].
[00:58:42]      |
[00:58:42]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:58:42] 
[00:58:48]     Finished release [optimized] target(s) in 22.77s
---
[00:58:54]  Documenting std v0.0.0 (/checkout/src/libstd)
[00:59:00] error: `[char::escape_debug]` cannot be resolved, ignoring it...
[00:59:00]     --> /checkout/src/libcore/str/mod.rs:3968:67
[00:59:00]      |
[00:59:00] 3968 |     /// Return an iterator that escapes each char in `self` with [`char::escape_debug`].
[00:59:00]      |
[00:59:00] note: lint level defined here
[00:59:00]     --> src/libstd/lib.rs:209:9
[00:59:00]      |
[00:59:00]      |
[00:59:00] 209  | #![deny(intra_doc_link_resolution_failure)]
[00:59:00]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:00]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:59:00] 
[00:59:00] error: `[char::escape_default]` cannot be resolved, ignoring it...
[00:59:00]      |
[00:59:00]      |
[00:59:00] 4014 |     /// Return an iterator that escapes each char in `self` with [`char::escape_default`].
[00:59:00]      |
[00:59:00]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:59:00] 
[00:59:00] error: `[char::escape_unicode]` cannot be resolved, ignoring it...
[00:59:00] error: `[char::escape_unicode]` cannot be resolved, ignoring it...
[00:59:00]     --> /checkout/src/libcore/str/mod.rs:4050:67
[00:59:00]      |
[00:59:00] 4050 |     /// Return an iterator that escapes each char in `self` with [`char::escape_unicode`].
[00:59:00]      |
[00:59:00]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:59:00] 
[00:59:01] error: Could not document `std`.
[00:59:01] error: Could not document `std`.
[00:59:01] 
[00:59:01] Caused by:
[00:59:01]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std src/libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="backtrace-sys"' --cfg 'feature="compiler_builtins"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-9e6c0311b71511c6.rmeta --extern backtrace_sys=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-f13b165ed9b4dd57.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d54fe968dea87029.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-84e5b9599b1b7754.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-a69fda92b07aedd5.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-349a3e5cce9f18ee.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-d7c48504ff1056b6.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-20814bca47e9a554.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-8bc4c4bf1d4ec8f0.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-0814373332ff833c.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-655667bc8a1522d0.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-5756fbda854daecf.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-5e31c590860d0940.rmeta` (exit code: 1)
[00:59:01] 
[00:59:01] 
[00:59:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--index-page" "/checkout/src/doc/index.md"
[00:59:01] 
[00:59:01] 
[00:59:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:59:01] Build completed unsuccessfully in 0:05:58
[00:59:01] Build completed unsuccessfully in 0:05:58
[00:59:01] make: *** [all] Error 1
[00:59:01] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a70c8b2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 15:12:45 UTC 2019
---
travis_time:end:00705538:start=1550675566434693651,finish=1550675566439225313,duration=4531662
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06507058
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b03afba
travis_time:start:0b03afba
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01b60e61
$ dmesg | grep -i kill
