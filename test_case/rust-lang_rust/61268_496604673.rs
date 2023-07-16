plain
travis_time:end:0dcd936a:start=1559056887674898160,finish=1559056888433409042,duration=758510882
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
[01:20:41] 
[01:20:41] running 143 tests
[01:20:44] i..iii.....iii..iiii.....i......................i...i................i......i.........ii.i..i..i.ii. 100/143
[01:20:46] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:20:46] 
[01:20:46]  finished in 4.696
[01:20:46] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:48] 
[01:20:48] running 9 tests
[01:20:48] iiiiiiiii
[01:20:48] 
[01:20:48]  finished in 0.158
[01:20:48] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:04] 
[01:21:04] running 122 tests
[01:21:30] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:21:35] .i.i......iii.i.....ii
[01:21:35] 
[01:21:35]  finished in 30.513
[01:21:35] travis_fold:end:test_debuginfo

---
[01:45:25] 
[01:45:25] stdout ----
[01:45:25] 
[01:45:25] running 1 test
[01:45:25] test /checkout/src/doc/rustc/src/profile-guided-optimization.md - Profile_Guided_Optimization::Usage (line 43) ... FAILED
[01:45:25] failures:
[01:45:25] 
[01:45:25] 
[01:45:25] ---- /checkout/src/doc/rustc/src/profile-guided-optimization.md - Profile_Guided_Optimization::Usage (line 43) stdout ----
[01:45:25] error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `component`
[01:45:25]  --> /checkout/src/doc/rustc/src/profile-guided-optimization.md:44:8
[01:45:25] 3 | rustup component add llvm-tools-preview
[01:45:25]   |        ^^^^^^^^^ expected one of 8 possible tokens here
[01:45:25] 
[01:45:25] error[E0425]: cannot find value `rustup` in this scope
---
[01:45:25] 
[01:45:25] error: aborting due to 2 previous errors
[01:45:25] 
[01:45:25] For more information about this error, try `rustc --explain E0425`.
[01:45:25] thread '/checkout/src/doc/rustc/src/profile-guided-optimization.md - Profile_Guided_Optimization::Usage (line 43)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:320:13
[01:45:25] 
[01:45:25] 
[01:45:25] failures:
[01:45:25] failures:
[01:45:25]     /checkout/src/doc/rustc/src/profile-guided-optimization.md - Profile_Guided_Optimization::Usage (line 43)
[01:45:25] test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:45:25] 
[01:45:25] 
[01:45:25] stderr ----
[01:45:25] stderr ----
[01:45:25] 
[01:45:25] 
[01:45:25] 
[01:45:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:25] Build completed unsuccessfully in 0:36:45
[01:45:25] make: *** [check] Error 1
[01:45:25] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:354b0800
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May 28 17:07:05 UTC 2019
---
travis_time:end:094dae8e:start=1559063227275306941,finish=1559063227280379075,duration=5072134
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e441d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d57b450
travis_time:start:0d57b450
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0985bd58
$ dmesg | grep -i kill
