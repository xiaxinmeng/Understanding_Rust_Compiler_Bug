plain
tidy check
[00:04:38] * 555 error codes
[00:04:38] * highest error code: E0712
[00:04:38] * 228 features
[00:04:39] Stray file with UI testing output: "/checkout/src/test/ui/directory_ownership/unowned_mod_with_path.stderr"
[00:04:39] Stray file with UI testing output: "/checkout/src/test/ui/directory_ownership/mod_file_not_owning.stderr"
[00:04:39] Stray file with UI testing output: "/checkout/src/test/ui/non_modrs_mods/non_modrs_mods.stderr"
[00:04:39] some tidy checks failed
[00:04:39] 
[00:04:39] 
[00:04:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:39] 
[00:04:39] 
[00:04:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:39] Build completed unsuccessfully in 0:00:50
[00:04:39] Build completed unsuccessfully in 0:00:50
[00:04:39] Makefile:79: recipe for target 'tidy' failed
[00:04:39] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f6b8dc8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:08393c73:start=1536384335377410383,finish=1536384335385891591,duration=8481208
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:026976f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09e63e27
travis_time:start:09e63e27
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05f55430
$ dmesg | grep -i kill
