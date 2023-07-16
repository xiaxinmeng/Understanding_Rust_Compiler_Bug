plain

[00:05:13] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:995: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:996: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:997: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:998: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:999: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1000: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1001: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1002: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1003: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1004: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1005: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1006: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1007: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1008: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1009: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1010: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/librustdoc/html/markdown.rs:1011: line longer than 100 chars
[00:05:14] some tidy checks failed
[00:05:14] 
[00:05:14] 
[00:05:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:14] 
[00:05:14] 
[00:05:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:14] Build completed unsuccessfully in 0:00:51
[00:05:14] Build completed unsuccessfully in 0:00:51
[00:05:14] Makefile:79: recipe for target 'tidy' failed
[00:05:14] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a4685c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00a77d62:start=1537356554546756779,finish=1537356554550792357,duration=4035578
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3502c8be
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0722234a
travis_time:start:0722234a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:004b6340
$ dmesg | grep -i kill
