plain
travis_time:end:2387f0e2:start=1555853738249474647,finish=1555853824135756931,duration=85886282284
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
[01:16:35] 
[01:16:35] running 9 tests
[01:16:35] iiiiiiiii
[01:16:35] 
[01:16:35]  finished in 0.147
[01:16:35] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:51] 
[01:16:51] running 121 tests
[01:17:17] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:17:22] i.i......iii.i.....ii
[01:17:22] 
[01:17:22]  finished in 30.493
[01:17:22] travis_fold:end:test_debuginfo

---
[01:42:53]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:43:04] error[E0308]: mismatched types
[01:43:04]     --> src/libsyntax/print/pprust.rs:3249:21
[01:43:04]      |
[01:43:04] 3249 | /                     ast::FnHeader {
[01:43:04] 3250 | |                         unsafety: ast::Unsafety::Normal,
[01:43:04] 3251 | |                         constness: source_map::dummy_spanned(ast::Constness::NotConst),
[01:43:04] 3252 | |                         asyncness: source_map::dummy_spanned(ast::IsAsync::NotAsync),
[01:43:04] 3253 | |                         abi: Abi::Rust,
[01:43:04] 3254 | |                     },
[01:43:04]      | |_____________________^ expected reference, found struct `ast::FnHeader`
[01:43:04]      |
[01:43:04]      = note: expected type `&ast::FnHeader`
[01:43:04]                 found type `ast::FnHeader`
[01:43:04]      |
[01:43:04]      |
[01:43:04] 3249 |                     &ast::FnHeader {
[01:43:04] 3250 |                         unsafety: ast::Unsafety::Normal,
[01:43:04] 3251 |                         constness: source_map::dummy_spanned(ast::Constness::NotConst),
[01:43:04] 3252 |                         asyncness: source_map::dummy_spanned(ast::IsAsync::NotAsync),
[01:43:04] 3253 |                         abi: Abi::Rust,
[01:43:04]      |
[01:43:04] 
[01:43:05] error: aborting due to previous error
[01:43:05] 
[01:43:05] 
[01:43:05] For more information about this error, try `rustc --explain E0308`.
[01:43:05] error: Could not compile `syntax`.
[01:43:05] 
[01:43:05] To learn more, run the command again with --verbose.
[01:43:05] 
[01:43:05] 
[01:43:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:43:05] 
[01:43:05] 
[01:43:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:43:05] Build completed unsuccessfully in 0:38:34
[01:43:05] Build completed unsuccessfully in 0:38:34
[01:43:05] Makefile:48: recipe for target 'check' failed
[01:43:05] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:041b7de8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 21 15:20:18 UTC 2019
---
travis_time:end:1d5fb14c:start=1555860020188025543,finish=1555860020193282136,duration=5256593
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:078cc00c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c759de6
travis_time:start:0c759de6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15f99f1e
$ dmesg | grep -i kill
