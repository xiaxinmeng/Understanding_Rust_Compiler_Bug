plain
[00:55:34] test [ui] ui/rfc1445/match-forbidden-without-eq.rs ... ok
[00:55:34] test [ui] ui/rfc1598-generic-associated-types/construct_with_other_type.rs ... ok
[00:55:34] test [ui] ui/rfc1598-generic-associated-types/empty_generics.rs ... ok
[00:55:34] test [ui] ui/rfc1598-generic-associated-types/collections.rs ... ok
[00:55:34] test [ui] ui/rfc1598-generic-associated-types/gat-dont-ice-on-absent-feature.rs ... ok
[00:55:34] test [ui] ui/rfc1598-generic-associated-types/generic-associated-types-where.rs ... ok
[00:55:35] test [ui] ui/rfc1598-generic-associated-types/iterable.rs ... ok
[00:55:35] test [ui] ui/rfc1598-generic-associated-types/generic_associated_type_undeclared_lifetimes.rs ... ok
[00:55:35] test [ui] ui/rfc1598-generic-associated-types/gat-incomplete-warning.rs ... ok
---
[00:56:09] failures:
[00:56:09] 
[00:56:09] ---- [ui] ui/consts/static_mut_containing_mut_ref3.rs stdout ----
[00:56:09] 
[00:56:09] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:56:09] status: exit code: 101
[00:56:09] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/static_mut_containing_mut_ref3.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/static_mut_containing_mut_ref3/auxiliary" "-A" "unused"
[00:56:09] ------------------------------------------
[00:56:09] 
[00:56:09] ------------------------------------------
[00:56:09] stderr:
[00:56:09] stderr:
[00:56:09] ------------------------------------------
[00:56:09] error: internal compiler error: src/librustc_mir/interpret/place.rs:604: eval_place_to_mplace called on ((FOO: (u8, u8)).0: u8)
[00:56:09] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
[00:56:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:56:09] error: aborting due to previous error
[00:56:09] 
[00:56:09] 
[00:56:09] 
[00:56:09] note: the compiler unexpectedly panicked. this is a bug.
[00:56:09] 
[00:56:09] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:09] 
[00:56:09] note: rustc 1.36.0-nightly (db04f4967 2019-05-24) running on x86_64-unknown-linux-gnu
[00:56:09] 
[00:56:09] note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0 -C linker=cc
[00:56:09] 
[00:56:09] ------------------------------------------
[00:56:09] 
[00:56:09] 
---
[00:56:09] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[00:56:09] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:56:09] 
[00:56:09] 
[00:56:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.36.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:09] 
[00:56:09] 
[00:56:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:56:09] Build completed unsuccessfully in 0:52:41
---
travis_time:end:17fca645:start=1558725691285299059,finish=1558725691301038352,duration=15739293
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:078b9e68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c233b89
travis_time:start:0c233b89
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09c0a455
$ dmesg | grep -i kill
