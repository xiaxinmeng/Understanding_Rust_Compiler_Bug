plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:33:19] 
[01:33:19] running 186 tests
[01:34:07] ....................i..................................F............................................
[01:34:57] .....................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:36:05] failures:
[01:36:05] 
[01:36:05] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:36:05] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:36:05] ---- [run-make] run-make-fulldeps/hotplug_codegen_backend stdout ----
[01:36:05] 
[01:36:05] error: make failed
[01:36:05] status: exit code: 2
[01:36:05] command: "make"
[01:36:05] stdout:
[01:36:05] ------------------------------------------
[01:36:05] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:36:05] /bin/echo || exit 0 # This test requires /bin/echo to exist
[01:36:05] 
[01:36:05] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
[01:36:05]  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
[01:36:05] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  some_crate.rs --crate-name some_crate --crate-type bin -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/some_crate \
[01:36:05]  -Z codegen-backend=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib -Z unstable-options
[01:36:05] Makefile:4: recipe for target 'all' failed
[01:36:05] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:36:05] ------------------------------------------
[01:36:05] stderr:
[01:36:05] ------------------------------------------
[01:36:05] ------------------------------------------
[01:36:05] warning: ignoring --out-dir flag due to -o flag
[01:36:05] 
[01:36:05] warning: ignoring --out-dir flag due to -o flag
[01:36:05] error[E0463]: can't find crate for `std`
[01:36:05] 
[01:36:05] error: aborting due to previous error
[01:36:05] 
[01:36:05] 
[01:36:05] For more information about this error, try `rustc --explain E0463`.
[01:36:05] make[1]: *** [all] Error 101
[01:36:05] ------------------------------------------
[01:36:05] 
[01:36:05] 
[01:36:05] thread '[run-make] run-make-fulldeps/hotplug_codegen_backend' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:36:05] 
[01:36:05] 
[01:36:05] failures:
[01:36:05]     [run-make] run-make-fulldeps/hotplug_codegen_backend
[01:36:05]     [run-make] run-make-fulldeps/hotplug_codegen_backend
[01:36:05] 
[01:36:05] test result: FAILED. 184 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:36:05] 
[01:36:05] 
[01:36:05] 
[01:36:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:36:05] 
[01:36:05] 
[01:36:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:05] Build completed unsuccessfully in 0:48:48
[01:36:05] Build completed unsuccessfully in 0:48:48
[01:36:05] make: *** [check] Error 1
[01:36:05] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1328307e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:3223134c:start=1529344319606126779,finish=1529344319615321779,duration=9195000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:049417b6
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:13b0a7fa
$ dmesg | grep -i kill
