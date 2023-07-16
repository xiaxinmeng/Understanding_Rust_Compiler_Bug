plain
[01:37:06] status: exit code: 2
[01:37:06] command: "make"
[01:37:06] stdout:
[01:37:06] ------------------------------------------
[01:37:06] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/a-b-a-linker-guard'
[01:37:06] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu  a.rs --cfg x -C prefer-dynamic
[01:37:06] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu  b.rs -C prefer-dynamic
[01:37:06] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu/b
[01:37:06] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu  a.rs --cfg y -C prefer-dynamic
[01:37:06] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/a-b-a-linker-guard.stage2-x86_64-unknown-linux-gnu/b && exit 1 || exit 0
[01:37:06] Makefile:8: recipe for target 'all' failed
[01:37:06] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/a-b-a-linker-guard'
[01:37:06] ------------------------------------------
[01:37:06] stderr:
[01:37:06] ------------------------------------------
[01:37:06] warning: unused variable: `x`
[01:37:06] warning: unused variable: `x`
[01:37:06]   --> a.rs:15:12
[01:37:06]    |
[01:37:06] 15 | pub fn foo(x: u32) { }
[01:37:06]    |            ^ help: consider using `_x` instead
[01:37:06]    = note: #[warn(unused_variables)] on by default
[01:37:06] 
[01:37:06] warning: unused variable: `x`
[01:37:06]   --> a.rs:18:12
[01:37:06]   --> a.rs:18:12
[01:37:06]    |
[01:37:06] 18 | pub fn foo(x: i32) { }
[01:37:06]    |            ^ help: consider using `_x` instead
[01:37:06]    = note: #[warn(unused_variables)] on by default
[01:37:06] 
[01:37:06] 
[01:37:06] make[1]: *** [all] Error 1
[01:37:06] ------------------------------------------
[01:37:06] 
[01:37:06] 
[01:37:06] thread '[run-make] run-make-fulldeps/a-b-a-linker-guard' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:37:06] 
[01:37:06] 
[01:37:06] failures:
[01:37:06]     [run-make] run-make-fulldeps/a-b-a-linker-guard
[01:37:06]     [run-make] run-make-fulldeps/a-b-a-linker-guard
[01:37:06] 
[01:37:06] test result: FAILED. 182 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:37:06] 
[01:37:06] 
[01:37:06] 
[01:37:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:37:06] 
[01:37:06] 
[01:37:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:37:06] Build completed unsuccessfully in 0:50:11
[01:37:06] Build completed unsuccessfully in 0:50:11
[01:37:06] Makefile:58: recipe for target 'check' failed
[01:37:06] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d4f3338
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
