plain

[00:16:43] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:995: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:996: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:997: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:998: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:999: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1000: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1001: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1002: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1003: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1004: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1005: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1006: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1007: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1008: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1009: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1010: line longer than 100 chars
[00:16:43] tidy error: /checkout/src/librustdoc/html/markdown.rs:1011: line longer than 100 chars
[00:16:44] some tidy checks failed
[00:16:44] 
[00:16:44] 
[00:16:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:16:44] 
[00:16:44] 
[00:16:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:16:44] Build completed unsuccessfully in 0:00:54
[00:16:44] Build completed unsuccessfully in 0:00:54
[00:16:44] make: *** [tidy] Error 1
[00:16:44] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e6f62fa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:044f1894:start=1537359160426317532,finish=1537359160431011349,duration=4693817
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16e4b260
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:039ae2c0
travis_time:start:039ae2c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e64fd30
$ dmesg | grep -i kill
