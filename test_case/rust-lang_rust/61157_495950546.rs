plain
travis_time:end:10dfcfe8:start=1558814688581140561,finish=1558814777780621821,duration=89199481260
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
[01:19:51] 
[01:19:51] running 143 tests
[01:19:54] i..iii.....iii..iiii.....i......................i...i................i.....i..........ii.i..i..i.ii. 100/143
[01:19:55] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:19:55] 
[01:19:55]  finished in 4.598
[01:19:55] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:57] 
[01:19:57] running 9 tests
[01:19:57] iiiiiiiii
[01:19:57] 
[01:19:57]  finished in 0.149
[01:19:57] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:13] 
[01:20:13] running 122 tests
[01:20:38] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:20:43] .i.i......iii.i.....ii
[01:20:43] 
[01:20:43]  finished in 30.100
[01:20:43] travis_fold:end:test_debuginfo

---
[01:32:10] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:32:10]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:32:31] error: function is never used: `test_buffered_reader_seek_underflow_discard_buffer_between_seeks`
[01:32:31]      |
[01:32:31]      |
[01:32:31] 1165 |     fn test_buffered_reader_seek_underflow_discard_buffer_between_seeks() {
[01:32:31]      |
[01:32:31]      = note: `-D dead-code` implied by `-D warnings`
[01:32:31] 
[01:32:32] error: aborting due to previous error
[01:32:32] error: aborting due to previous error
[01:32:32] 
[01:32:32] error: Could not compile `std`.
[01:32:32] 
[01:32:32] To learn more, run the command again with --verbose.
[01:32:32] 
[01:32:32] 
[01:32:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:32:32] 
[01:32:32] 
[01:32:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:32] Build completed unsuccessfully in 0:24:34
[01:32:32] Build completed unsuccessfully in 0:24:34
[01:32:32] Makefile:48: recipe for target 'check' failed
[01:32:32] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1e28b616
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May 25 21:38:59 UTC 2019
---
travis_time:end:139bf697:start=1558820341431618849,finish=1558820341437339023,duration=5720174
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:240454ce
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0366f060
travis_time:start:0366f060
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fca9e4c
$ dmesg | grep -i kill
