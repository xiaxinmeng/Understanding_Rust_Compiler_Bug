plain
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:54] 
[01:26:54] running 187 tests
[01:27:39] ....................i...............................................................................
[01:28:30] .........................................................................F............test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:29:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:29:37] failures:
[01:29:37] 
[01:29:37] ---- [run-make] run-make-fulldeps/target-without-atomics stdout ----
[01:29:37] ---- [run-make] run-make-fulldeps/target-without-atomics stdout ----
[01:29:37] 
[01:29:37] error: make failed
[01:29:37] status: exit code: 2
[01:29:37] command: "make"
[01:29:37] stdout:
[01:29:37] ------------------------------------------
[01:29:37] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/target-without-atomics'
[01:29:37] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-without-atomics/target-without-atomics:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-without-atomics/target-without-atomics -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-without-atomics/target-without-atomics  --print cfg --target thumbv6m-none-eabi | "/checkout/src/etc/cat-and-grep.sh" -v target_has_atomic
[01:29:37] [[[ begin stdout ]]]
[01:29:37] debug_assertions
[01:29:37] target_arch="arm"
[01:29:37] target_endian="little"
[01:29:37] target_env=""
[01:29:37] target_has_atomic="16"
[01:29:37] target_has_atomic="32"
[01:29:37] target_has_atomic="8"
[01:29:37] target_has_atomic="ptr"
[01:29:37] target_os="none"
[01:29:37] target_pointer_width="32"
[01:29:37] target_vendor=""
[01:29:37] 
[01:29:37] [[[ end stdout ]]]
[01:29:37] Error: should not match: target_has_atomic
[01:29:37] Makefile:5: recipe for target 'all' failed
[01:29:37] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/target-without-atomics'
[01:29:37] ------------------------------------------
[01:29:37] stderr:
[01:29:37] ------------------------------------------
[01:29:37] ------------------------------------------
[01:29:37] make[1]: *** [all] Error 1
[01:29:37] ------------------------------------------
[01:29:37] 
[01:29:37] 
[01:29:37] thread '[run-make] run-make-fulldeps/target-without-atomics' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3140:9
[01:29:37] 
[01:29:37] 
[01:29:37] failures:
[01:29:37]     [run-make] run-make-fulldeps/target-without-atomics
[01:29:37]     [run-make] run-make-fulldeps/target-without-atomics
[01:29:37] 
[01:29:37] test result: FAILED. 185 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
[01:29:37] 
[01:29:37] 
[01:29:37] 
[01:29:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:29:37] 
[01:29:37] 
[01:29:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:29:37] Build completed unsuccessfully in 0:46:56
[01:29:37] Build completed unsuccessfully in 0:46:56
[01:29:37] Makefile:58: recipe for target 'check' failed
[01:29:37] make: *** [check] Error 1
sckowoo9/s-f2msl497wd-46px1w-2tbvj9xpkkcf2
122008 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
115724 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
104364 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
104360 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
