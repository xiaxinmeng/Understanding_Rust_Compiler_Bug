plain
travis_time:end:0a1b26f4:start=1558262949460467000,finish=1558263035593101403,duration=86132634403
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:05:01]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[01:05:01]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:05:01]     Checking hashbrown v0.3.0
[01:05:02]  Documenting std v0.0.0 (/checkout/src/libstd)
[01:05:08] error: `[Weak::from_raw]` cannot be resolved, ignoring it...
[01:05:08]      |
[01:05:08]      |
[01:05:08] 1310 |     /// can be turned back into the `Weak<T>` with [`from_raw`][Weak::from_raw].
[01:05:08]      |
[01:05:08] note: lint level defined here
[01:05:08]     --> src/libstd/lib.rs:211:9
[01:05:08]      |
[01:05:08]      |
[01:05:08] 211  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
[01:05:08]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] error: `[Weak::from_raw]` cannot be resolved, ignoring it...
[01:05:08]      |
[01:05:08]      |
[01:05:08] 1310 |     /// can be turned back into the `Weak<T>` with [`from_raw`][Weak::from_raw].
[01:05:08]      |
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] 
[01:05:08] error: `[Weak::as_raw]` cannot be resolved, ignoring it...
[01:05:08]      |
[01:05:08]      |
[01:05:08] 1313 |     /// [`as_raw`][Weak::as_raw] apply.
[01:05:08]      |
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] 
[01:05:08] error: `[Weak::into_raw]` cannot be resolved, ignoring it...
[01:05:08]      |
[01:05:08]      |
[01:05:08] 1339 |     /// Converts a raw pointer previously created by [`into_raw`][Weak::into_raw] back into
[01:05:08]      |
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] error: `[Weak::upgrade]` cannot be resolved, ignoring it...
[01:05:08] error: `[Weak::upgrade]` cannot be resolved, ignoring it...
[01:05:08]     --> /checkout/src/liballoc/rc.rs:1342:83
[01:05:08]      |
[01:05:08] 1342 |     /// This can be used to safely get a strong reference (by calling [`upgrade`][Weak::upgrade]
[01:05:08]      |
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] error: `[Weak]` cannot be resolved, ignoring it...
[01:05:08] error: `[Weak]` cannot be resolved, ignoring it...
[01:05:08]     --> /checkout/src/liballoc/rc.rs:1345:89
[01:05:08]      |
[01:05:08] 1345 |     /// It takes ownership of one weak count. In case a [`null`] is passed, a dangling [`Weak`] is
[01:05:08]      |
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] 
[01:05:08] error: `[Rc]` cannot be resolved, ignoring it...
[01:05:08]      |
[01:05:08]      |
[01:05:08] 1351 |     /// is or *was* managed by an [`Rc`] and the weak count of that [`Rc`] must not have reached
[01:05:08]      |
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] 
[01:05:08] error: `[Rc]` cannot be resolved, ignoring it...
[01:05:08]      |
[01:05:08]      |
[01:05:08] 1351 |     /// is or *was* managed by an [`Rc`] and the weak count of that [`Rc`] must not have reached
[01:05:08]      |
[01:05:08]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:05:08] 
[01:05:08] error: aborting due to 8 previous errors
[01:05:08] error: aborting due to 8 previous errors
[01:05:08] 
[01:05:08] error: Could not document `std`.
[01:05:08] 
[01:05:08] Caused by:
[01:05:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name std src/libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="alloc"' --cfg 'feature="backtrace"' --cfg 'feature="backtrace-sys"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.36.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-10941456f404e667.rmeta --extern backtrace_sys=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-4c14e92add76a922.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-42bb64a0001037a7.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-298f01c3cfd4145b.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-2fc02c4647fd63ba.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-3b6cd7efc73905d4.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-47fb0e4c106b0176.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-c5d84c7d9aee2d71.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-b222a28e9750aefd.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-4a2c3b52852efb00.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-bff536f4904ed9d8.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-aa62eda555423254.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-8b2540fc12b9af3e.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-ea2e0915f4ceb155.rmeta` (exit code: 1)
[01:05:08] 
[01:05:08] 
[01:05:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.36.0" "--index-page" "/checkout/src/doc/index.md"
[01:05:08] 
[01:05:08] 
[01:05:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:05:08] Build completed unsuccessfully in 0:07:51
---
travis_time:end:0affb670:start=1558266954778799472,finish=1558266954785022722,duration=6223250
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1165fb3c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12e98680
travis_time:start:12e98680
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06cce7b8
$ dmesg | grep -i kill
