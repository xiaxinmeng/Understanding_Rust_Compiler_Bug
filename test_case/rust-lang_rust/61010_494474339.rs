plain
travis_time:end:1f96b5ed:start=1558452131578142973,finish=1558452132554474418,duration=976331445
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
[01:19:41] 
[01:19:41] running 143 tests
[01:19:43] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:19:45] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:45] 
[01:19:45]  finished in 4.573
[01:19:45] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:47] 
[01:19:47] running 9 tests
[01:19:47] iiiiiiiii
[01:19:47] 
[01:19:47]  finished in 0.154
[01:19:47] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:02] 
[01:20:02] running 122 tests
[01:20:27] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:20:32] .i.i......iii.i.....ii
[01:20:32] 
[01:20:32]  finished in 29.767
[01:20:32] travis_fold:end:test_debuginfo

---
[01:37:17]     Finished release [optimized] target(s) in 15.16s
[01:37:17]      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_data_structures-8e968bbe17ec4ce7
[01:37:17] 
[01:37:17] running 142 tests
[01:37:17] ....F............................................................................................... 100/142
[01:37:17] failures:
[01:37:17] 
[01:37:17] ---- bit_set::hybrid_bitset stdout ----
[01:37:17] ---- bit_set::hybrid_bitset stdout ----
[01:37:17] thread 'bit_set::hybrid_bitset' panicked at 'assertion failed: hybrid.union(&dense256)', src/librustc_data_structures/bit_set.rs:989:5
[01:37:17] 
[01:37:17] 
[01:37:17] failures:
[01:37:17]     bit_set::hybrid_bitset
[01:37:17]     bit_set::hybrid_bitset
[01:37:17] 
[01:37:17] test result: FAILED. 141 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:37:17] 
[01:37:17] error: test failed, to rerun pass '--lib'
[01:37:17] 
[01:37:17] 
[01:37:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_data_structures" "--" "--quiet"
[01:37:17] 
[01:37:17] 
[01:37:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:37:17] Build completed unsuccessfully in 0:29:11
[01:37:17] Build completed unsuccessfully in 0:29:11
[01:37:17] make: *** [check] Error 1
[01:37:17] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1ae30c11
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 21 16:59:41 UTC 2019
---
travis_time:end:0d30b787:start=1558457983164593256,finish=1558457983168946839,duration=4353583
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:083d527a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0294dad0
travis_time:start:0294dad0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:25b99800
$ dmesg | grep -i kill
