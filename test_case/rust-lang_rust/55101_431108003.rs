plain

[00:03:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:58] tidy error: /checkout/src/libsyntax/feature_gate.rs:1659: TODO is deprecated; use FIXME
[00:03:58] tidy error: /checkout/src/librustc/ty/instance.rs:309: TODO is deprecated; use FIXME
[00:03:58] tidy error: /checkout/src/librustc/traits/project.rs:1530: TODO is deprecated; use FIXME
[00:03:59] some tidy checks failed
[00:03:59] 
[00:03:59] 
[00:03:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:59] 
[00:03:59] 
[00:03:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:59] Build completed unsuccessfully in 0:00:45
[00:03:59] Build completed unsuccessfully in 0:00:45
[00:03:59] Makefile:79: recipe for target 'tidy' failed
[00:03:59] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13462db8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:088e90cc:start=1539885644673659132,finish=1539885644678289543,duration=4630411
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:002900c6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1774a6fa
travis_time:start:1774a6fa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e3a322f
$ dmesg | grep -i kill
