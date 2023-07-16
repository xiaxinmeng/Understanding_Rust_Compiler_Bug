plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:07] 
[01:17:07] running 187 tests
[01:17:37] ....................F........................F......................................................
[01:18:30] ......................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
 'staticlib' failed
[01:19:12] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/cross-lang-lto'
[01:19:12] ------------------------------------------
[01:19:12] stderr:
[01:19:12] ------------------------------------------
[01:19:12] ------------------------------------------
[01:19:12] warning: ignoring --out-dir flag due to -o flag
[01:19:12] 
[01:19:12] ReportError reading '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/cross-lang-lto/cross-lang-lto/liblib.lib0.rcgu.o': No such file or directory
[01:19:12] make[1]: *** [staticlib] Error 1
[01:19:12] ------------------------------------------
[01:19:12] 
[01:19:12] 
[01:19:12] thread '[run-make] run-make-fulldeps/cross-lang-lto' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3163:9
[01:19:12] 
[01:19:12] ---- [run-make] run-make-fulldeps/extra-filename-with-temp-outputs stdout ----
[01:19:12] 
[01:19:12] error: make failed
[01:19:12] error: make failed
[01:19:12] status: exit code: 2
[01:19:12] command: "make"
[01:19:12] stdout:
[01:19:12] ------------------------------------------
[01:19:12] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/extra-filename-with-temp-outputs'
[01:19:12] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extra-filename-with-temp-outputs/extra-filename-with-temp-outputs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extra-filename-with-temp-outputs/extra-filename-with-temp-outputs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extra-filename-with-temp-outputs/extra-filename-with-temp-outputs  -C extra-filename=bar foo.rs -C save-temps
[01:19:12] rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extra-filename-with-temp-outputs/extra-filename-with-temp-outputs/foobar.foo0.rcgu.o
[01:19:12] Makefile:4: recipe for target 'all' failed
[01:19:12] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/extra-filename-with-temp-outputs'
[01:19:12] ------------------------------------------
[01:19:12] stderr:
[01:19:12] ------------------------------------------
[01:19:12] warning: `-C save-temps` might not produce all requested temporary products when incremental compilation is enabled.
[01:19:12] warning: `-C save-temps` might not produce all requested temporary products when incremental compilation is enabled.
[01:19:12] 
[01:19:12] rm: cannot remove '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extra-filename-with-temp-outputs/extra-filename-with-temp-outputs/foobar.foo0.rcgu.o': No such file or directory
[01:19:12] make[1]: *** [all] Error 1
[01:19:12] ------------------------------------------
[01:19:12] 
[01:19:12] thread '[run-make] run-make-fulldeps/extra-filename-with-temp-outputs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3163:9
[01:19:12] 
---
[01:19:12] test result: FAILED. 185 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:19:12] 
[01:19:12] 
[01:19:12] 
[01:19:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter binaryformat bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:12] 
[01:19:12] 
[01:19:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:12] Build completed unsuccessfully in 0:33:30
[01:19:12] Build completed unsuccessfully in 0:33:30
[01:19:12] Makefile:58: recipe for target 'check' failed
[01:19:12] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2d25a078
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
