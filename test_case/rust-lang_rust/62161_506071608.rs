plain
travis_time:end:014c790e:start=1561582960022547486,finish=1561582960777692844,duration=755145358
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:31] 
[01:06:31] running 9 tests
[01:06:31] iiiiiiiii
[01:06:31] 
[01:06:31]  finished in 0.151
[01:06:31] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:47] 
[01:06:47] running 122 tests
[01:07:12] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:07:17] .i.i......iii.i.....ii
[01:07:17] 
[01:07:17]  finished in 29.723
[01:07:17] travis_fold:end:test_debuginfo

---
[01:39:24]     Checking compiler_builtins v0.1.16
[01:39:25]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[01:39:29]     Finished release [optimized] target(s) in 34.64s
[01:39:30]  Documenting core v0.0.0 (/checkout/src/libcore)
[01:39:45] error: `[i32::MAX]` cannot be resolved, ignoring it...
[01:39:45]     |
[01:39:45]     |
[01:39:45] 417 | /// giving the [`i64`]'s value modulo [`i32::MAX`]) or by simply returning
[01:39:45]     |
[01:39:45] note: lint level defined here
[01:39:45]    --> src/libcore/lib.rs:64:9
[01:39:45]     |
[01:39:45]     |
[01:39:45] 64  | #![deny(intra_doc_link_resolution_failure)] // rustdoc is run without -D warnings
[01:39:45]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:39:45]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:39:45] 
[01:39:45] error: `[i32::MAX]` cannot be resolved, ignoring it...
[01:39:45]     |
[01:39:45]     |
[01:39:45] 418 | /// [`i32::MAX`], or by some other method. The [`From`] trait is intended
[01:39:45]     |
[01:39:45]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:39:45] 
[01:39:45] error: `[]` cannot be resolved, ignoring it...
[01:39:45] error: `[]` cannot be resolved, ignoring it...
[01:39:45]    --> src/libcore/convert.rs:429:15
[01:39:45]     |
[01:39:45] 429 | /// When the [`!`] type is stablized [`Infallible`] and [`!`] will be
[01:39:45]     |
[01:39:45]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:39:45] 
[01:39:45] error: `[]` cannot be resolved, ignoring it...
[01:39:45] error: `[]` cannot be resolved, ignoring it...
[01:39:45]    --> src/libcore/convert.rs:429:58
[01:39:45]     |
[01:39:45] 429 | /// When the [`!`] type is stablized [`Infallible`] and [`!`] will be
[01:39:45]     |
[01:39:45]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:39:45] 
[01:39:46] error: aborting due to 4 previous errors
[01:39:46] error: aborting due to 4 previous errors
[01:39:46] 
[01:39:46] error: Could not document `core`.
[01:39:46] 
[01:39:46] Caused by:
[01:39:46]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name core src/libcore/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.37.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[01:39:46] 
[01:39:46] 
[01:39:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.37.0" "--index-page" "/checkout/src/doc/index.md"
[01:39:46] 
[01:39:46] 
[01:39:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:46] Build completed unsuccessfully in 1:34:56
---
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_time:end:252a12f8:start=1561588959415059357,finish=1561588959483785904,duration=68726547
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2eda56d4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:000c795c
$ dmesg | grep -i kill
