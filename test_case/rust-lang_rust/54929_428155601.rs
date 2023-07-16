plain

[00:05:01] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:223: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:259: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:310: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:337: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:364: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:375: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:391: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:422: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:518: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:588: line longer than 100 chars
[00:05:02] tidy error: /checkout/src/libsyntax/attr/builtin.rs:621: line longer than 100 chars
[00:05:03] some tidy checks failed
[00:05:03] 
[00:05:03] 
[00:05:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:03] 
[00:05:03] 
[00:05:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:03] Build completed unsuccessfully in 0:00:46
[00:05:03] Build completed unsuccessfully in 0:00:46
[00:05:03] Makefile:79: recipe for target 'tidy' failed
[00:05:03] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:037cfd90
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:233bd6ae:start=1539084082831056489,finish=1539084082835498600,duration=4442111
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0144a072
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15904f40
travis_time:start:15904f40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0076e126
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.463250] init: failsafe main process (1094) killed by TERM signal
[   41.878416] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
