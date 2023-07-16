plain
[00:02:36]  Downloading proc-macro2 v0.4.13
[00:02:36]  Downloading syn v0.14.8
[00:02:36]  Downloading quote v0.6.6
[00:02:36]  Downloading unicode-xid v0.1.0
[00:02:36]  Downloading ryu v0.2.4
[00:02:37]  Downloading cfg-if v0.1.5
[00:02:37]    Compiling proc-macro2 v0.4.13
[00:02:37]    Compiling unicode-xid v0.1.0
[00:02:37]    Compiling serde v1.0.72
[00:02:37]    Compiling serde v1.0.72
[00:02:37]    Compiling ryu v0.2.4
[00:02:37]    Compiling ordermap v0.3.5
[00:02:37]    Compiling cc v1.0.19
[00:02:37]    Compiling fixedbitset v0.1.9
[00:02:37]    Compiling cfg-if v0.1.5
---
travis_time:start:stage0-tidy
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
[00:03:53]    Compiling proc-macro2 v0.4.13
[00:03:53]    Compiling unicode-xid v0.1.0
[00:03:53]    Compiling ryu v0.2.4
[00:03:53]    Compiling itoa v0.4.2
[00:03:58]    Compiling quote v0.6.6
[00:03:58]    Compiling syn v0.14.8
[00:04:06]    Compiling serde_json v1.0.26
---
[00:04:42] * version_check 
[00:04:42] some tidy checks failed
[00:04:42] 
[00:04:42] 
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:42] 
[00:04:42] 
[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:42] Build completed unsuccessfully in 0:00:50
[00:04:42] Build completed unsuccessfully in 0:00:50
[00:04:42] Makefile:79: recipe for target 'tidy' failed
[00:04:42] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ec371f6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0a31d984:start=1534872728843148954,finish=1534872728851543356,duration=8394402
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d8573b2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:151cd4d8
travis_time:start:151cd4d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:191d691b
$ dmesg | grep -i kill
