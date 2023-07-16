plain
[00:47:25]    Compiling env_logger v0.5.8
[00:47:25]    Compiling rustfix v0.2.0
[00:47:29]    Compiling compiletest v0.0.0 (file:///checkout/src/tools/compiletest)
[00:47:54] ..................................................................................i.................
[00:48:00] .............F...................i..................................................................
[00:48:08] ....................................................................................................
[00:48:11] ....................................................................................................
[00:48:15] ....................................................................................................
[00:48:27] ....................................................................................................
---
[00:49:04] failures:
[00:49:04] 
[00:49:04] ---- [ui] ui/const-eval/duration_conversion.rs stdout ----
[00:49:04]  
[00:49:04] error: test compilation failed although it shouldn't!
[00:49:04] status: exit code: 101
[00:49:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/duration_conversion.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/duration_conversion.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/duration_conversion.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:49:04] ------------------------------------------
[00:49:04] 
[00:49:04] ------------------------------------------
[00:49:04] stderr:
[00:49:04] stderr:
[00:49:04] ------------------------------------------
[00:49:04] {"message":"`std::time::Duration::as_secs` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/duration_conversion.rs","byte_start":820,"byte_end":841,"line_start":20,"line_end":20,"column_start":25,"column_end":46,"is_primary":true,"text":[{"text":"    const _ONE: usize = _ONE_SECOND.as_secs() as usize;","highlight_start":25,"highlight_end":46}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `std::time::Duration::as_secs` is not yet stable as a const fn\n  --> /checkout/src/test/ui/const-eval/duration_conversion.rs:20:25\n   |\nLL |     const _ONE: usize = _ONE_SECOND.as_secs() as usize;\n   |                         ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable\n\n"}
[00:49:04] {"message":"`std::time::Duration::subsec_millis` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/duration_conversion.rs","byte_start":876,"byte_end":908,"line_start":21,"line_end":21,"column_start":25,"column_end":57,"is_primary":true,"text":[{"text":"    const _TWO: usize = _ONE_MILLISECOND.subsec_millis() as usize;","highlight_start":25,"highlight_end":57}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `std::time::Duration::subsec_millis` is not yet stable as a const fn\n  --> /checkout/src/test/ui/const-eval/duration_conversion.rs:21:25\n   |\nLL |     const _TWO: usize = _ONE_MILLISECOND.subsec_millis() as usize;\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable\n\n"}
[00:49:04] {"message":"`std::time::Duration::subsec_micros` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/duration_conversion.rs","byte_start":945,"byte_end":977,"line_start":22,"line_end":22,"column_start":27,"column_end":59,"is_primary":true,"text":[{"text":"    const _THREE: usize = _ONE_MICROSECOND.subsec_micros() as usize;","highlight_start":27,"highlight_end":59}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `std::time::Duration::subsec_micros` is not yet stable as a const fn\n  --> /checkout/src/test/ui/const-eval/duration_conversion.rs:22:27\n   |\nLL |     const _THREE: usize = _ONE_MICROSECOND.subsec_micros() as usize;\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable\n\n"}
[00:49:04] {"message":"`std::time::Duration::subsec_nanos` is not yet stable as a const fn","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/duration_conversion.rs","byte_start":1013,"byte_end":1043,"line_start":23,"line_end":23,"column_start":26,"column_end":56,"is_primary":true,"text":[{"text":"    const _FOUR: usize = _ONE_NANOSECOND.subsec_nanos() as usize;","highlight_start":26,"highlight_end":56}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `std::time::Duration::subsec_nanos` is not yet stable as a const fn\n  --> /checkout/src/test/ui/const-eval/duration_conversion.rs:23:26\n   |\nLL |     const _FOUR: usize = _ONE_NANOSECOND.subsec_nanos() as usize;\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: in Nightly builds, add `#![feature(duration_getters)]` to the crate attributes to enable\n\n"}
[00:49:04] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:49:04] ------------------------------------------
[00:49:04] 
[00:49:04] thread '[ui] ui/const-eval/duration_conversion.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:49:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:49:04] 
[00:49:04] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:49:04] 
[00:49:04] 
[00:49:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:04] 
[00:49:04] 
[00:49:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:04] Build completed unsuccessfully in 0:02:36
[00:49:04] Build completed unsuccessfully in 0:02:36
[00:49:04] make: *** [check] Error 1
[00:49:04] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3136d883
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
