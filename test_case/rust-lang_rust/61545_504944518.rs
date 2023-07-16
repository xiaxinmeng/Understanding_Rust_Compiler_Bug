plain
travis_time:end:13fd3df2:start=1561366018366694228,finish=1561366020836060792,duration=2469366564
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
[01:03:17] 
[01:03:17] running 9 tests
[01:03:17] iiiiiiiii
[01:03:17] 
[01:03:17]  finished in 0.147
[01:03:17] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:33] 
[01:03:33] running 122 tests
[01:03:57] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:02] .i.i......iii.i.....ii
[01:04:02] 
[01:04:02]  finished in 29.558
[01:04:02] travis_fold:end:test_debuginfo

---
[01:12:57]    Compiling semver v0.9.0
[01:12:57]    Compiling rustc_version v0.2.3
[01:12:57] error[E0602]: unknown lint: `rustc::internal`
[01:12:57]   |
[01:12:57]   = note: requested on the command line with `-W rustc::internal`
[01:12:58] error: aborting due to previous error
[01:12:58] 
[01:12:58] For more information about this error, try `rustc --explain E0602`.
[01:12:58] error: Could not compile `rustc_version`.
[01:12:58] error: Could not compile `rustc_version`.
[01:12:58] 
[01:12:58] To learn more, run the command again with --verbose.
[01:12:58] 
[01:12:58] 
[01:12:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:12:58] 
[01:12:58] 
[01:12:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:58] Build completed unsuccessfully in 1:08:20
---
travis_time:end:28ae8130:start=1561370410859619732,finish=1561370410924808637,duration=65188905
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01aa4d60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:331497a8
$ dmesg | grep -i kill
