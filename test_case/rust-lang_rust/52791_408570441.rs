plain
[00:07:37] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "fabricate", path: "src/tools/rust-installer", mode: ToolBootstrap, is_optional_tool: false, source_type: Submodule, extra_features: [] } -- 49.492
[00:11:18] [TIMING] PlainSourceTarball -- 350.922
[00:11:18] Dist src
[00:11:29] [TIMING] Src -- 10.774
[00:11:29] tar (child): /checkout/obj/build/dist/rustc-1.29.tar.gz: Cannot open: No such file or directory
[00:11:29] tar (child): Error is not recoverable: exiting now
[00:11:29] tar: Child returned status 2
[00:11:29] 
[00:11:29] 
[00:11:29] 
[00:11:29] command did not execute successfully: "tar" "-xzf" "/checkout/obj/build/dist/rustc-1.29.tar.gz" "--strip-components=1"
[00:11:29] 
[00:11:29] 
[00:11:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:11:29] Build completed unsuccessfully in 0:08:53
---
travis_time:end:03e29200:start=1532738768386605545,finish=1532738768392228625,duration=5623080
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0772c8ce
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00ca8ae8
travis_time:start:00ca8ae8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a3d0b04
$ dmesg | grep -i kill
