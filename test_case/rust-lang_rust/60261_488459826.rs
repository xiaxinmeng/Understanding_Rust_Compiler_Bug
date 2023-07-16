plain
travis_time:end:163ac152:start=1556743076735159207,finish=1556743077552593399,duration=817434192
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
[01:22:53] 
[01:22:53] running 9 tests
[01:22:53] iiiiiiiii
[01:22:53] 
[01:22:53]  finished in 0.162
[01:22:53] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:09] 
[01:23:09] running 121 tests
[01:23:35] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:23:40] i.i......iii.i.....ii
[01:23:40] 
[01:23:40]  finished in 30.665
[01:23:40] travis_fold:end:test_debuginfo

---
[01:44:15]   |
[01:44:15] 1 | ::abc::def::r#return
[01:44:15]   |             ^^^^^^^^
[01:44:15] 
[01:44:16] ..................................F...................................................
[01:44:16] 
[01:44:16] 
[01:44:16] ---- parse::unescape::tests::test_unescape_byte_bad stdout ----
[01:44:16] thread 'parse::unescape::tests::test_unescape_byte_bad' panicked at 'assertion failed: `(left == right)`
[01:44:16]   left: `Err(UnicodeEscapeInByte)`,
[01:44:16]  right: `Err(OverlongUnicodeEscape)`', src/libsyntax/parse/unescape.rs:401:13
[01:44:16] 
[01:44:16] 
[01:44:16] failures:
[01:44:16]     parse::unescape::tests::test_unescape_byte_bad
[01:44:16]     parse::unescape::tests::test_unescape_byte_bad
[01:44:16] 
[01:44:16] test result: FAILED. 85 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:44:16] 
[01:44:16] error: test failed, to rerun pass '--lib'
[01:44:16] 
[01:44:16] 
[01:44:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:44:16] 
[01:44:16] 
[01:44:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:16] Build completed unsuccessfully in 0:33:24
[01:44:16] Build completed unsuccessfully in 0:33:24
[01:44:16] Makefile:48: recipe for target 'check' failed
[01:44:16] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:370cf593
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May  1 22:22:25 UTC 2019
---
travtravis_time:end:019678c8:start=1556749347135529931,finish=1556749347202430456,duration=66900525
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:039cc2f7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18dde5f0
$ dmesg | grep -i kill
