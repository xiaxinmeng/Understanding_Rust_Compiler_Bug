plain
[00:02:15]   Downloaded env_logger v0.5.13
[00:02:15]   Downloaded pkg-config v0.3.14
[00:02:16]   Downloaded socket2 v0.3.8
[00:02:16]   Downloaded proc-macro2 v0.3.8
[00:02:16]   Downloaded miniz_oxide_c_api v0.2.0
[00:02:16]   Downloaded vergen v3.0.4
[00:02:16]   Downloaded num-traits v0.2.6
[00:02:16]   Downloaded ammonia v1.1.0
[00:02:16]   Downloaded phf_codegen v0.7.22
---
[01:28:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[01:28:49] expected success, got: exit code: 101
[01:28:49] 
[01:28:49] 
[01:28:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:28:49] Build completed unsuccessfully in 0:26:52
[01:28:49] make: *** [check-aux] Error 1
[01:28:49] Makefile:60: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21a63398
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec  9 08:19:35 UTC 2018
---
travis_time:end:026010f0:start=1544343577600384255,finish=1544343577608659835,duration=8275580
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0118b7c5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:036da440
travis_time:start:036da440
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05fe1a00
$ dmesg | grep -i kill
