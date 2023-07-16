plain
travis_time:end:014e60e2:start=1559831420945941179,finish=1559831421903615115,duration=957673936
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
[01:02:47] 
[01:02:47] running 144 tests
[01:02:49] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:02:51] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:02:51] 
[01:02:51]  finished in 4.499
[01:02:51] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:53] 
[01:02:53] running 9 tests
[01:02:53] iiiiiiiii
[01:02:53] 
[01:02:53]  finished in 0.150
[01:02:53] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:09] 
[01:03:09] running 122 tests
[01:03:33] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:03:38] .i.i......iii.i.....ii
[01:03:38] 
[01:03:38]  finished in 29.673
[01:03:38] travis_fold:end:test_debuginfo

---
[01:17:00] 
[01:17:00]    Doc-tests alloc
[01:17:00] 
[01:17:00] running 388 tests
[01:17:19] ...............................................F.................................................... 100/388
[01:17:44] .................................................................................................... 300/388
[01:17:54] ........................................................................................
[01:17:54] failures:
[01:17:54] 
[01:17:54] 
[01:17:54] ---- collections/btree/map.rs - collections::btree::map::BTreeMap<K, V>::range_mut (line 841) stdout ----
[01:17:54] error[E0283]: type annotations required: cannot resolve `_: std::cmp::Ord`
[01:17:54]   --> collections/btree/map.rs:848:25
[01:17:54]    |
[01:17:54] 10 | for (_, balance) in map.range_mut("B".."Cheryl") {
[01:17:54] 
[01:17:54] error: aborting due to previous error
[01:17:54] 
[01:17:54] For more information about this error, try `rustc --explain E0283`.
---
[01:17:54] 
[01:17:54] error: test failed, to rerun pass '--doc'
[01:17:54] 
[01:17:54] 
[01:17:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:17:54] 
[01:17:54] 
[01:17:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:17:54] Build completed unsuccessfully in 1:13:19
---
travis_time:end:067d2620:start=1559836109091911892,finish=1559836109143247983,duration=51336091
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1394d64b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16c8e7df
$ dmesg | grep -i kill
