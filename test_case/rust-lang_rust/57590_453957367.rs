plain
[00:05:36]  ---> f2d758482422
[00:05:36] Step 3/49 : WORKDIR /build
[00:05:36]  ---> Using cache
[00:05:36]  ---> ce0fba60179c
[00:05:36] Step 4/49 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
[00:05:36]  ---> c97f3431e7cc
[00:05:36] Step 5/49 : COPY dist-various-1/build-rumprun.sh /build
[00:05:36]  ---> Using cache
[00:05:36]  ---> 948411f72c3d
---
[00:58:17] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 86.849
[00:58:17] travis_fold:start:test_run-make
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv6m-none-eabi)
[00:58:17] thread 'main' panicked at 'Cannot determine OS from triple', src/tools/compiletest/src/util.rs:77:5
[00:58:17] 
[00:58:17] 
[00:58:17] 
[00:58:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:17] 
[00:58:17] 
[00:58:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:58:17] Build completed unsuccessfully in 0:50:57
---
travis_time:end:12ca00cd:start=1547461311891254181,finish=1547461311904412830,duration=13158649
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:17c52420
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03e54c84
travis_time:start:03e54c84
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02d54a80
$ dmesg | grep -i kill
