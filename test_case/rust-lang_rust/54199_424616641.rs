plain
  0     0    0     0    0     0      0      0 --:--:--  0:00:18 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:00:19 --:--:--     0curl: (6) Could not resolve host: s3-us-west-1.amazonaws.com
[01:15:16] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:621:17
[01:15:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:15:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:15:16] Build completed unsuccessfully in 0:12:06
[01:15:16] Makefile:60: recipe for target 'check-aux' failed
[01:15:16] make: *** [check-aux] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0711d0c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:232703d0:start=1537947379036373993,finish=1537947379042112347,duration=5738354
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d5625a6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ee99866
travis_time:start:0ee99866
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:006b857d
$ dmesg | grep -i kill
