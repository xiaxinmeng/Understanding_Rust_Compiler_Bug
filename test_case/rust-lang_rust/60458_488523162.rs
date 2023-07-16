plain
travis_time:end:1428c5e8:start=1556754481005641291,finish=1556754568799967674,duration=87794326383
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:59]     Checking core v0.0.0 (/checkout/src/libcore)
[01:04:28]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[01:04:29]     Checking compiler_builtins v0.1.10
[01:04:30]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[01:04:33] error: `[value]` cannot be resolved, ignoring it...
[01:04:33]    --> /checkout/src/libcore/fmt/builders.rs:722:37
[01:04:33]     |
[01:04:33] 722 |     /// This method, together with [`value`] are an alternative to [`entry`] that
[01:04:33]     |
[01:04:33] note: lint level defined here
[01:04:33]    --> src/liballoc/lib.rs:64:9
[01:04:33]     |
[01:04:33]     |
[01:04:33] 64  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
[01:04:33]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:04:33]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:04:33] 
[01:04:33] error: `[entry]` cannot be resolved, ignoring it...
[01:04:33]    --> /checkout/src/libcore/fmt/builders.rs:722:69
[01:04:33]     |
[01:04:33] 722 |     /// This method, together with [`value`] are an alternative to [`entry`] that
[01:04:33]     |
[01:04:33]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:04:33] 
[01:04:33] 
[01:04:33] error: `[finish]` cannot be resolved, ignoring it...
[01:04:33]    --> /checkout/src/libcore/fmt/builders.rs:727:56
[01:04:33]     |
[01:04:33] 727 |     /// by a corresponding call to `value`. Otherwise [`finish`] will return an error.
[01:04:33]     |
[01:04:33]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:04:33] 
[01:04:33] 
[01:04:33] error: `[key]` cannot be resolved, ignoring it...
[01:04:33]    --> /checkout/src/libcore/fmt/builders.rs:781:37
[01:04:33]     |
[01:04:33] 781 |     /// This method, together with [`key`] are an alternative to [`entry`] that
[01:04:33]     |
[01:04:33]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:04:33] 
[01:04:33] 
[01:04:33] error: `[entry]` cannot be resolved, ignoring it...
[01:04:33]    --> /checkout/src/libcore/fmt/builders.rs:781:67
[01:04:33]     |
[01:04:33] 781 |     /// This method, together with [`key`] are an alternative to [`entry`] that
[01:04:33]     |
[01:04:33]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:04:33] 
[01:04:33] 
[01:04:33] error: `[finish]` cannot be resolved, ignoring it...
[01:04:33]    --> /checkout/src/libcore/fmt/builders.rs:786:56
[01:04:33]     |
[01:04:33] 786 |     /// by a corresponding call to `value`. Otherwise [`finish`] will return an error.
[01:04:33]     |
[01:04:33]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:04:33] 
[01:04:33] error: aborting due to 6 previous errors
[01:04:33] error: aborting due to 6 previous errors
[01:04:33] 
[01:04:33] error: Could not document `alloc`.
[01:04:33] 
[01:04:33] Caused by:
[01:04:33]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name alloc src/liballoc/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.36.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-c48a38dddb046cd4.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-72f00ef37db6d7ed.rmeta` (exit code: 1)
[01:04:33] 
[01:04:33] 
[01:04:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.36.0" "--index-page" "/checkout/src/doc/index.md"
[01:04:33] 
[01:04:33] 
[01:04:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:04:33] Build completed unsuccessfully in 0:07:36
---
travis_time:end:0125f002:start=1556758454464045058,finish=1556758454469730992,duration=5685934
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0710e67a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04f9b7a6
travis_time:start:04f9b7a6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:115858e0
$ dmesg | grep -i kill
