plain
travis_time:end:00d4f79c:start=1559872029610227710,finish=1559872031890594390,duration=2280366680
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:10] 
[01:05:10] running 144 tests
[01:05:13] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:05:15] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:15] 
[01:05:15]  finished in 4.634
[01:05:15] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:17] 
[01:05:17] running 9 tests
[01:05:17] iiiiiiiii
[01:05:17] 
[01:05:17]  finished in 0.156
[01:05:17] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:33] 
[01:05:33] running 122 tests
[01:05:59] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:04] .i.i......iii.i.....ii
[01:06:04] 
[01:06:04]  finished in 30.925
[01:06:04] travis_fold:end:test_debuginfo

---
[01:25:48]     Finished release [optimized] target(s) in 1.26s
[01:25:48]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/fmt_macros-2711e98f75b71201
[01:25:48] 
[01:25:48] running 14 tests
[01:25:48] ......F.......
[01:25:48] 
[01:25:48] ---- tests::format_counts stdout ----
[01:25:48] ---- tests::format_counts stdout ----
[01:25:48] thread 'tests::format_counts' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
[01:25:48] 
[01:25:48] 
[01:25:48] failures:
[01:25:48]     tests::format_counts
[01:25:48]     tests::format_counts
[01:25:48] 
[01:25:48] test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:25:48] 
[01:25:48] error: test failed, to rerun pass '-p fmt_macros --lib'
[01:25:48] 
[01:25:48] 
[01:25:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "fmt_macros" "--" "--quiet"
[01:25:48] 
[01:25:48] 
[01:25:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:48] Build completed unsuccessfully in 1:21:49
