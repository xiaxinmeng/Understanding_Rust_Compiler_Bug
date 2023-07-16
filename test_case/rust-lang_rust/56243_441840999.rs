plain
travis_time:end:25b6b36a:start=1543269077208130635,finish=1543269131050956164,duration=53842825529
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:58] .................................................................................................... 100/5067
[00:49:01] .................................................................................................... 200/5067
[00:49:04] .............................ii............................................ii...................ii.. 300/5067
[00:49:07] ..............................................................................................iii... 400/5067
[00:49:10] .....iiiiiiii.iii............................iii...........................................i........ 500/5067
[00:49:17] .................................................................................................... 700/5067
[00:49:23] ................................................................................................i... 800/5067
[00:49:27] ........i........................................................................................... 900/5067
[00:49:30] ...............iiiii..................ii.iiii....................................................... 1000/5067
[00:49:30] ...............iiiii..................ii.iiii....................................................... 1000/5067
[00:49:33] ..........................................................................................iiiiiiii.. 1100/5067
[00:49:37] .................................................................................................... 1300/5067
[00:49:40] .................................................................................................... 1400/5067
[00:49:42] .............................................i...................................................... 1500/5067
[00:49:45] ..............i.........ii.........................................................i................ 1600/5067
---
[00:50:11] .................................................................................................... 2300/5067
[00:50:15] .................................................................................................... 2400/5067
[00:50:18] .................................................................................................... 2500/5067
[00:50:22] .................................................................................................... 2600/5067
[00:50:25] ...iiiiiiiii........................................................................................ 2700/5067
[00:50:32] .................................................................................................... 2900/5067
[00:50:36] .................................................................................................... 3000/5067
[00:50:40] ..................................................................i................................. 3100/5067
[00:50:43] .................................................................................................... 3200/5067
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:15] 
[01:05:15] running 117 tests
[01:05:18] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:05:18] i.i.....iiii.....
[01:05:18] 
[01:05:18]  finished in 3.333
[01:05:18] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:33] 
[01:05:33] running 118 tests
[01:05:58] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:02] ......iii.i.....ii
[01:06:02] 
[01:06:02]  finished in 29.056
[01:06:02] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:38:39] 
[01:38:39] running 192 tests
[01:39:05] ..........................................................................................F......... 100/192
[01:40:01] ...........................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:40:59] failures:
[01:40:59] 
[01:40:59] ---- [run-make] run-make-fulldeps/libtest-json stdout ----
[01:40:59] 
[01:40:59] 
[01:40:59] error: make failed
[01:40:59] status: exit code: 2
[01:40:59] command: "make"
[01:40:59] stdout:
[01:40:59] ------------------------------------------
[01:40:59] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/libtest-json'
[01:40:59] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json  --test f.rs
[01:40:59] RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json || true
[01:40:59] cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json | "/usr/bin/python2.7" validate_json.py
[01:40:59] # Compare to output file
[01:40:59] diff output.json /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/libtest-json/libtest-json/libtest-json-output.json
[01:40:59] 5c5
[01:40:59] < { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'b' panicked at 'assertion failed: false', f.rs:18:5\nnote: Run with `RUST_BACKTRACE=1` for a backtrace.\n" }
[01:40:59] ---
[01:40:59] > { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'main' panicked at 'assertion failed: false', f.rs:18:5\nnote: Run with `RUST_BACKTRACE=1` for a backtrace.\n" }
[01:40:59] Makefile:8: recipe for target 'all' failed
[01:40:59] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/libtest-json'
[01:40:59] ------------------------------------------
[01:40:59] stderr:
[01:40:59] ------------------------------------------
[01:40:59] ------------------------------------------
[01:40:59] make[1]: *** [all] Error 1
[01:40:59] ------------------------------------------
[01:40:59] 
[01:40:59] thread '[run-make] run-make-fulldeps/libtest-json' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:40:59] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:40:59] 
[01:40:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:40:59] 
[01:40:59] 
[01:40:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:40:59] 
[01:40:59] 
[01:40:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:59] Build completed unsuccessfully in 0:56:08
[01:40:59] Build completed unsuccessfully in 0:56:08
[01:40:59] Makefile:58: recipe for target 'check' failed
[01:40:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0772a6ac
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Nov 26 23:33:19 UTC 2018
---
travis_time:end:0fcbb581:start=1543275202042607434,finish=1543275202048820043,duration=6212609
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:011c0fc4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0adcd714
travis_time:start:0adcd714
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:089967a8
$ dmesg | grep -i kill
