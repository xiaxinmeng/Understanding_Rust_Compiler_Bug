plain

[01:24:28] travis_time:end:stage0-linkchecker:start=1538940267499346160,finish=1538940269785948454,duration=2286602294

[01:24:28] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "linkchecker", path: "src/tools/linkchecker", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 2.287
[01:26:45] std/convert/trait.From.html:318: broken link - std/convert/struct.LocalWaker.html
[01:26:45] std/convert/trait.From.html:318: broken link - std/convert/struct.Waker.html
[01:27:08] core/convert/trait.From.html:67: broken link - core/convert/struct.LocalWaker.html
[01:27:08] core/convert/trait.From.html:67: broken link - core/convert/struct.Waker.html
[01:28:00] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:28:00] 
[01:28:00] 
[01:28:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:28:00] expected success, got: exit code: 101
[01:28:00] expected success, got: exit code: 101
[01:28:00] 
[01:28:00] 
[01:28:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:00] Build completed unsuccessfully in 0:43:45
[01:28:00] Makefile:58: recipe for target 'check' failed
[01:28:00] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01ea2aad
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00a51977:start=1538940493862311019,finish=1538940493872628388,duration=10317369
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:37e300c6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19af462d
travis_time:start:19af462d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0348026a
$ dmesg | grep -i kill
