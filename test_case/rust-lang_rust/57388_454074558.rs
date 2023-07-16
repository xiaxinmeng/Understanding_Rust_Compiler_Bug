plain
travis_fold:start:worker_info
Worker information
hostname: 959d582b-8d62-4f1c-b7ed-c8c4c1a080b8@1.production-2-worker-com-gce-fr58
version: v6.2.0 https://github.com/travis-ci/worker/tree/5e5476e01646095f48eec13196fdb3faf8f5cbf7
instance: travis-job-043bb802-a7ae-4250-9d1e-780f77996f00 travis-ci-stevonnie-xenial-1547455581-2c98a19 (via amqp)
travis_fold:end:worker_info
travis_fold:start:system_info
Build system information
Build language: shell
---
nvm version
0.34.0
perlbrew version
/home/travis/perl5/perlbrew/bin/perlbrew  - App::perlbrew/0.85
phpenv version
rbenv 1.1.1-39-g59785f6
rvm 1.29.7 (latest) by Michal Papis, Piotr Kuczynski, Wayne E. Seguin [https://rvm.io]
default ruby version
ruby 2.5.3p105 (2018-10-18 revision 65156) [x86_64-linux]
travis_fold:end:system_info
---
[00:03:08]  ---> f2d758482422
[00:03:08] Step 3/49 : WORKDIR /build
[00:03:08]  ---> Using cache
[00:03:08]  ---> ce0fba60179c
[00:03:08] Step 4/49 : RUN add-apt-repository ppa:team-gcc-arm-embedded/ppa &&     apt-get update &&     apt-get install -y --no-install-recommends gcc-arm-embedded
[00:03:08]  ---> c97f3431e7cc
[00:03:08] Step 5/49 : COPY dist-various-1/build-rumprun.sh /build
[00:03:08]  ---> Using cache
[00:03:08]  ---> 948411f72c3d
---
[00:57:04] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "compiletest", path: "src/tools/compiletest", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 90.229
[00:57:04] travis_fold:start:test_run-make
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv6m-none-eabi)
[00:57:04] thread 'main' panicked at 'Cannot determine OS from triple', src/tools/compiletest/src/util.rs:77:5
[00:57:04] 
[00:57:04] 
[00:57:04] 
[00:57:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:04] 
[00:57:04] 
[00:57:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
[00:57:04] Build completed unsuccessfully in 0:52:13
---
travis_time:end:004b3598:start=1547484309919015907,finish=1547484309962541273,duration=43525366
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00102f4b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0046017c
travis_time:start:0046017c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1906ccf5
$ dmesg | grep -i kill
