plain
travis_time:end:020a9d38:start=1560784114842155964,finish=1560784115798356393,duration=956200429
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
[01:10:31] 
[01:10:31] running 9 tests
[01:10:31] iiiiiiiii
[01:10:31] 
[01:10:31]  finished in 0.154
[01:10:31] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:47] 
[01:10:47] running 122 tests
[01:11:12] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:11:17] .i.i......iii.i.....ii
[01:11:17] 
[01:11:17]  finished in 29.823
[01:11:17] travis_fold:end:test_debuginfo

---
[01:23:09]    Compiling semver v0.9.0
[01:23:09]    Compiling rustc_version v0.2.3
[01:23:09] error[E0602]: unknown lint: `rustc::internal`
[01:23:09]   |
[01:23:09]   = note: requested on the command line with `-W rustc::internal`
[01:23:10] error: aborting due to previous error
[01:23:10] 
[01:23:10] For more information about this error, try `rustc --explain E0602`.
[01:23:10] error: Could not compile `rustc_version`.
[01:23:10] error: Could not compile `rustc_version`.
[01:23:10] 
[01:23:10] To learn more, run the command again with --verbose.
[01:23:10] 
[01:23:10] 
[01:23:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:23:10] 
[01:23:10] 
[01:23:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:10] Build completed unsuccessfully in 1:16:11
