plain
[01:12:44] travis_fold:end:stage0-linkchecker

[01:12:44] travis_time:end:stage0-linkchecker:start=1532696023062930756,finish=1532696025506231937,duration=2443301181

[01:12:56] reference/items/implementations.html:268: broken link fragment `#lint-check-attributes.html` pointing to `reference/attributes.html`
[01:12:56] reference/expressions/block-expr.html:205: broken link fragment `#lint-check-attributes.html` pointing to `reference/attributes.html`
[01:12:56] reference/print.html:2292: broken link fragment `#lint-check-attributes.html` pointing to `reference/attributes.html`
[01:12:56] reference/print.html:3485: broken link fragment `#lint-check-attributes.html` pointing to `reference/attributes.html`
[01:12:56] reference/print.html:3845: broken link fragment `#lint-check-attributes.html` pointing to `reference/attributes.html`
[01:12:56] reference/statements.html:227: broken link fragment `#lint-check-attributes.html` pointing to `reference/attributes.html`
[01:12:58] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:12:58] 
[01:12:58] 
[01:12:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:12:58] expected success, got: exit code: 101
[01:12:58] expected success, got: exit code: 101
[01:12:58] 
[01:12:58] 
[01:12:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:12:58] Build completed unsuccessfully in 0:30:31
[01:12:58] make: *** [check] Error 1
[01:12:58] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15946a16
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03414e70:start=1532696041014582096,finish=1532696041023499098,duration=8917002
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cac76c6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2f984988
travis_time:start:2f984988
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03449c0e
$ dmesg | grep -i kill
