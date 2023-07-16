plain
travis_time:end:074e9790:start=1556965254992540817,finish=1556965255780384680,duration=787843863
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:07:40]  Documenting core v0.0.0 (/checkout/src/libcore)
[01:07:56] error: `[borrow_mut]` cannot be resolved, ignoring it...
[01:07:56]    --> src/libcore/cell.rs:930:10
[01:07:56]     |
[01:07:56] 930 |     /// [`borrow_mut`] method instead if `self` isn't mutable.
[01:07:56]     |
[01:07:56] note: lint level defined here
[01:07:56]    --> src/libcore/lib.rs:64:9
[01:07:56]     |
[01:07:56]     |
[01:07:56] 64  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
[01:07:56]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:07:56]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:07:56] 
[01:07:56] error: `[borrow_mut]` cannot be resolved, ignoring it...
[01:07:56]    --> src/libcore/cell.rs:933:51
[01:07:56]     |
[01:07:56] 933 |     /// not what you want. In case of doubt, use [`borrow_mut`] instead.
[01:07:56]     |
[01:07:56]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:07:56] 
[01:07:57] error: aborting due to 2 previous errors
[01:07:57] error: aborting due to 2 previous errors
[01:07:57] 
[01:07:57] error: Could not document `core`.
[01:07:57] 
[01:07:57] Caused by:
[01:07:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name core src/libcore/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.36.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[01:07:57] 
[01:07:57] 
[01:07:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.36.0" "--index-page" "/checkout/src/doc/index.md"
[01:07:57] 
[01:07:57] 
[01:07:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:07:57] Build completed unsuccessfully in 0:07:49
---
travis_time:end:0b6bc508:start=1556969345678496687,finish=1556969345683963528,duration=5466841
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:112d37fe
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a754442
travis_time:start:0a754442
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07592aa5
$ dmesg | grep -i kill
