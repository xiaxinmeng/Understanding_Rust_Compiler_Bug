plain
travis_time:end:22cbbb80:start=1561155001507457737,finish=1561155005433717111,duration=3926259374
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
[01:04:55] 
[01:04:55] running 9 tests
[01:04:55] iiiiiiiii
[01:04:55] 
[01:04:55]  finished in 0.151
[01:04:55] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:11] 
[01:05:11] running 122 tests
[01:05:35] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:05:40] .i.i......iii.i.....ii
[01:05:40] 
[01:05:40]  finished in 29.485
[01:05:40] travis_fold:end:test_debuginfo

---
[01:23:52]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[01:23:53] error: hidden lifetime parameters in types are deprecated
[01:23:53]   --> src/librustc_data_structures/graph/test.rs:62:42
[01:23:53]    |
[01:23:53] 62 |     fn successors(&self, node: usize) -> <Self as GraphSuccessors>::Iter {
[01:23:53] 
[01:23:55] error: aborting due to previous error
[01:23:55] 
[01:23:55] error: Could not compile `rustc_data_structures`.
[01:23:55] error: Could not compile `rustc_data_structures`.
[01:23:55] 
[01:23:55] To learn more, run the command again with --verbose.
[01:23:55] 
[01:23:55] 
[01:23:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:23:55] 
[01:23:55] 
[01:23:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:55] Build completed unsuccessfully in 1:19:04
