plain

[00:04:27] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:27] tidy error: /checkout/src/libproc_macro/lib.rs:434: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_errors/lib.rs:58: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_metadata/decoder.rs:352: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_metadata/decoder.rs:353: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_lint/builtin.rs:198: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustdoc/html/highlight.rs:36: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustdoc/html/highlight.rs:46: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/libsyntax/parse/mod.rs:243: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:183: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/libsyntax/parse/lexer/mod.rs:224: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/libsyntax/source_map.rs:244: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/libsyntax/source_map.rs:300: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_typeck/check/compare_method.rs:322: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/librustc_typeck/check/method/suggest.rs:135: line longer than 100 chars
[00:04:28] some tidy checks failed
[00:04:28] 
[00:04:28] 
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:28] 
[00:04:28] 
[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:28] Build completed unsuccessfully in 0:00:50
[00:04:28] Build completed unsuccessfully in 0:00:50
[00:04:28] Makefile:79: recipe for target 'tidy' failed
[00:04:28] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:028d16fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:25918b7b:start=1534531497806196460,finish=1534531497814818953,duration=8622493
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:136f6a35
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10eefc09
travis_time:start:10eefc09
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08543352
$ dmesg | grep -i kill
