plain
travis_time:end:00587fc0:start=1554977160569481951,finish=1554977247893722900,duration=87324240949
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
[01:13:04] 
[01:13:04] running 9 tests
[01:13:04] iiiiiiiii
[01:13:04] 
[01:13:04]  finished in 0.149
[01:13:04] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:20] 
[01:13:20] running 121 tests
[01:13:45] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:13:49] i.i......iii.i.....ii
[01:13:49] 
[01:13:49]  finished in 29.804
[01:13:49] travis_fold:end:test_debuginfo

---
[01:34:12]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:34:38] error[E0308]: mismatched types
[01:34:38]     --> src/librustc/session/config.rs:3153:39
[01:34:38]      |
[01:34:38] 3153 |         opts.debugging_opts.pgo_gen = Some(String::from("abc"));
[01:34:38]      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `session::config::PgoGenerate`, found enum `std::option::Option`
[01:34:38]      |
[01:34:38]      = note: expected type `session::config::PgoGenerate`
[01:34:38] 
[01:34:47] error: aborting due to previous error
[01:34:47] 
[01:34:47] For more information about this error, try `rustc --explain E0308`.
[01:34:47] For more information about this error, try `rustc --explain E0308`.
[01:34:47] error: Could not compile `rustc`.
[01:34:47] 
[01:34:47] To learn more, run the command again with --verbose.
[01:34:47] 
[01:34:47] 
[01:34:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:34:47] 
[01:34:47] 
[01:34:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:47] Build completed unsuccessfully in 0:33:17
[01:34:47] Build completed unsuccessfully in 0:33:17
[01:34:47] make: *** [check] Error 1
[01:34:47] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d71c444
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 11 11:42:24 UTC 2019
