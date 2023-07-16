plain
[00:03:57] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "i686-unknown-linux-gnu" }, target: "i686-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 49.704
[00:03:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:57] tidy error: /checkout/src/libsyntax/parse/parser.rs:5690: line longer than 100 chars
[00:04:00] some tidy checks failed
[00:04:00] 
[00:04:00] 
[00:04:00] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:04:00] 
[00:04:00] 
[00:04:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:00] Build completed unsuccessfully in 0:00:54
[00:04:00] Build completed unsuccessfully in 0:00:54
[00:04:00] make: *** [tidy] Error 1
[00:04:00] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13269499
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr 29 01:22:01 UTC 2019
---
travis_time:end:1cad96d5:start=1556500921886114569,finish=1556500921891313396,duration=5198827
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:002d3847
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:032ffd78
travis_time:start:032ffd78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:017a070e
$ dmesg | grep -i kill
