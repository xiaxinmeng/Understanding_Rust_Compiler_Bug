plain
travis_time:end:0bede028:start=1554262138995228905,finish=1554262224773204907,duration=85777976002
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:44] 
[01:22:44] running 9 tests
[01:22:44] iiiiiiiii
[01:22:44] 
[01:22:44]  finished in 0.159
[01:22:44] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:01] 
[01:23:01] running 121 tests
[01:23:28] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:23:33] i.i......iii.i.....ii
[01:23:33] 
[01:23:33]  finished in 31.582
[01:23:33] travis_fold:end:test_debuginfo

---
[01:23:40] failures:
[01:23:40] 
[01:23:40] ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
[01:23:40] 
[01:23:40] error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" failed to compile: 
[01:23:40] status: exit code: 1
[01:23:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary"
[01:23:40] ------------------------------------------
[01:23:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:23:40] 
[01:23:40] ------------------------------------------
[01:23:40] ------------------------------------------
[01:23:40] stderr:
[01:23:40] ------------------------------------------
[01:23:40] {"message":"missing field `min_level` in initializer of `rustc::lint::Lint`","code":{"code":"E0063","explanation":"\nThis error indicates that during an attempt to build a struct or struct-like\nenum variant, one of the fields was not provided. Erroneous code example:\n\n