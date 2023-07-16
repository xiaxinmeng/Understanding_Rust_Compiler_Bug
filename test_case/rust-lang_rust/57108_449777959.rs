plain
[00:04:14] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "tidy", path: "src/tools/tidy", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 44.706
[00:04:14] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:14] tidy error: /checkout/src/test/ui/rust-2018/auxiliary/macro-use-warned-against2.rs: empty file
[00:04:14] tidy error: /checkout/src/test/ui/directory_ownership/foo/mod_file_not_owning/aux2.rs: empty file
[00:04:14] tidy error: /checkout/src/test/ui/directory_ownership/foo/mod_file_not_owning_aux2.rs: empty file
[00:04:14] tidy error: /checkout/src/test/ui/directory_ownership/mod_file_not_owning_aux1/mod_file_not_owning_aux2.rs: empty file
[00:04:14] tidy error: /checkout/src/test/ui/issues/issue-32709.rs: too many trailing newlines (3)
[00:04:14] tidy error: /checkout/src/test/ui/lint/auxiliary/lint_unused_extern_crate5.rs: empty file
[00:04:14] tidy error: /checkout/src/test/ui/lint/auxiliary/lint_unused_extern_crate4.rs: empty file
[00:04:14] tidy error: /checkout/src/test/run-make-fulldeps/extern-flag-fun/bar.rs: empty file
[00:04:14] tidy error: /checkout/src/test/run-make-fulldeps/mixing-formats/foo.rs: empty file
[00:04:14] tidy error: /checkout/src/test/run-make-fulldeps/no-intermediate-extras/foo.rs: empty file
[00:04:14] tidy error: /checkout/src/test/run-make-fulldeps/compiler-lookup-paths/native.c: empty file
[00:04:14] tidy error: /checkout/src/test/rustdoc/auxiliary/empty.rs: empty file
[00:04:16] some tidy checks failed
[00:04:16] 
[00:04:16] 
[00:04:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:04:16] 
[00:04:16] 
[00:04:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:16] Build completed unsuccessfully in 0:00:47
[00:04:16] Build completed unsuccessfully in 0:00:47
[00:04:16] Makefile:69: recipe for target 'tidy' failed
[00:04:16] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:056f7f16
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 24 23:26:42 UTC 2018
---
travis_time:end:0081e791:start=1545694003320079400,finish=1545694003327872613,duration=7793213
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a1a56be
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0caa09e5
travis_time:start:0caa09e5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:086e75c0
$ dmesg | grep -i kill
