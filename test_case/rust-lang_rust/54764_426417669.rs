plain

[00:04:11] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:11] tidy error: /checkout/src/test/debuginfo/struct-namespace.rs:27: trailing whitespace
[00:04:11] tidy error: /checkout/src/test/debuginfo/struct-namespace.rs:30: trailing whitespace
[00:04:11] tidy error: /checkout/src/test/debuginfo/simple-struct.rs:116: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/simple-struct.rs:120: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/simple-struct.rs:124: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/simple-struct.rs:128: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/destructured-for-loop-variable.rs:172: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/boxed-struct.rs:34: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/boxed-struct.rs:38: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/vec-slices.rs:104: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/evec-in-struct.rs:45: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/evec-in-struct.rs:48: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/evec-in-struct.rs:52: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/evec-in-struct.rs:60: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/tuple-struct.rs:62: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/c-style-enum.rs:123: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/c-style-enum.rs:127: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/c-style-enum.rs:131: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/c-style-enum.rs:135: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/generic-method-on-generic-struct.rs:77: line longer than 100 chars
[00:04:11] tidy error: /checkout/src/test/debuginfo/generic-method-on-generic-struct.rs:89: line longer than 100 chars
[00:04:13] some tidy checks failed
[00:04:13] 
[00:04:13] 
[00:04:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:13] 
[00:04:13] 
[00:04:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:13] Build completed unsuccessfully in 0:00:47
[00:04:13] Build completed unsuccessfully in 0:00:47
[00:04:13] make: *** [tidy] Error 1
[00:04:13] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02469270
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00365268:start=1538511416365293670,finish=1538511416370556172,duration=5262502
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05bf05de
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:141e4673
travis_time:start:141e4673
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:067e6afc
$ dmesg | grep -i kill
