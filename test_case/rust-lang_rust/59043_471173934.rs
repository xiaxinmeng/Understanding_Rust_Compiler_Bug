plain
[01:24:44] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 81.359
[01:24:44] travis_fold:start:test_ui
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> arm-linux-androideabi)
[01:24:44] thread 'main' panicked at 'UnrecognizedOption("Z")', src/tools/compiletest/src/main.rs:248:19
[01:24:44] 
[01:24:44] 
[01:24:44] 
[01:24:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "ui" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "-Zunstable-options" "--exclude-should-panic" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:24:44] 
[01:24:44] 
[01:24:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:24:44] Build completed unsuccessfully in 1:14:27
---
travis_time:end:1fb56a90:start=1552135374726677983,finish=1552135374744263386,duration=17585403
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0308d296
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:048f5fac
travis_time:start:048f5fac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:174fee08
$ dmesg | grep -i kill
