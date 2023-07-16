ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
[00:03:48] 
[00:03:48] 
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:43: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:47: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:49: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:51: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:55: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:58: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:61: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:63: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:65: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:69: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:72: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:78: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:83: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:89: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:94: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:96: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:102: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:104: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:110: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:112: trailing whitespace
[00:03:48] tidy error: /checkout/src/libcore/ops/drop.rs:116: trailing whitespace
[00:03:49] some tidy checks failed
[00:03:49] 
[00:03:49] 
[00:03:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:49] 
[00:03:49] 
[00:03:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:49] Build completed unsuccessfully in 0:00:47
[00:03:49] Build completed unsuccessfully in 0:00:47
[00:03:49] Makefile:69: recipe for target 'tidy' failed
[00:03:49] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00abbfd0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan  8 20:31:58 UTC 2019
---
travis_time:end:07f6ba6d:start=1546979519445687158,finish=1546979519449816042,duration=4128884
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0044d720
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:187cddd8
travis_time:start:187cddd8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0617cae0
$ dmesg | grep -i kill
