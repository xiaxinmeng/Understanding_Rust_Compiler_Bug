plain
travis_time:end:12604ba0:start=1559338092068475625,finish=1559338182397578818,duration=90329103193
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
[01:07:55] 
[01:07:55] running 143 tests
[01:07:58] i..iii.....iii..iiii.....i......................i..i.................i......i.........ii.i..i..i.ii. 100/143
[01:08:00] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:08:00] 
[01:08:00]  finished in 4.746
[01:08:00] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:02] 
[01:08:02] running 9 tests
[01:08:02] iiiiiiiii
[01:08:02] 
[01:08:02]  finished in 0.157
[01:08:02] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:18] 
[01:08:18] running 122 tests
[01:08:45] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:08:50] .i.i......iii.i.....ii
[01:08:50] 
[01:08:50]  finished in 31.777
[01:08:50] travis_fold:end:test_debuginfo

---
[01:47:02] 
[01:47:02] error[E0730]: cannot pattern-match on an array without a fixed length
[01:47:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:11760:9
[01:47:02]   |
[01:47:02] 7 |         [1, 2, 3] => true, // error: cannot pattern-match on an
[01:47:02] 
[01:47:02] error: aborting due to previous error
[01:47:02] 
[01:47:02] For more information about this error, try `rustc --explain E0730`.
[01:47:02] For more information about this error, try `rustc --explain E0730`.
[01:47:02] Some expected error codes were not found: ["E0527"]
[01:47:02] failures:
[01:47:02]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0730 (line 11755)
[01:47:02] 
[01:47:02] test result: FAILED. 678 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
---
travis_time:end:03445230:start=1559344616131439778,finish=1559344616137264352,duration=5824574
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09d1f404
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12b9e594
travis_time:start:12b9e594
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:272afec0
$ dmesg | grep -i kill
