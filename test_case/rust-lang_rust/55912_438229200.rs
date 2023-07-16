plain
[01:10:28] test [ui] ui/rfc1598-generic-associated-types/empty_generics.rs ... ok
[01:10:29] test [ui] ui/rfc1598-generic-associated-types/generic-associated-types-where.rs ... ok
[01:10:29] test [ui] ui/rfc1598-generic-associated-types/gat-incomplete-warning.rs ... ok
[01:10:29] test [ui] ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs ... ok
[01:10:29] test [ui] ui/rfc1445/phantom-data-is-structurally-matchable.rs ... ok
[01:10:29] test [ui] ui/rfc1598-generic-associated-types/parse/in-trait-impl.rs ... ok
[01:10:29] test [ui] ui/rfc1598-generic-associated-types/parse/in-trait.rs ... ok
[01:10:29] test [ui] ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs ... ok
[01:10:29] test [ui] ui/rfc1598-generic-associated-types/pointer_family.rs ... ok
---
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/collections.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/construct_with_other_type.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/empty_generics.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/generic-associated-types-where.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1445/phantom-data-is-structurally-matchable.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/gat-incomplete-warning.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/parse/in-trait-impl.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/parameter_number_and_kind.rs ... ok
[01:13:42] test [ui (nll)] ui/rfc1598-generic-associated-types/iterable.rs ... ok
---
[01:23:30] ---- [run-pass] run-pass/command-exec.rs stdout ----
[01:23:30] 
[01:23:30] error: test run failed!
[01:23:30] status: exit code: 101
[01:23:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/command-exec/a"
[01:23:30] ------------------------------------------
[01:23:30] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/command-exec/a", waiting for result
[01:23:30] 
[01:23:30] ------------------------------------------
[01:23:30] ------------------------------------------
[01:23:30] stderr:
[01:23:30] ------------------------------------------
[01:23:30] thread 'main' panicked at 'assertion failed: output.status.success()', /checkout/src/test/run-pass/command-exec.rs:99:5
[01:23:30] 
[01:23:30] ------------------------------------------
[01:23:30] 
[01:23:30] thread '[run-pass] run-pass/command-exec.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
---
[01:23:30] 
[01:23:30] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:23:30] 
[01:23:30] 
[01:23:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "run-pass" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:23:30] 
[01:23:30] 
[01:23:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:23:30] Build completed unsuccessfully in 1:12:34
---
travis_time:end:083854a2:start=1542107578928967950,finish=1542107578946335531,duration=17367581
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f680d60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0af3486e
travis_time:start:0af3486e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2f3f368f
$ dmesg | grep -i kill
