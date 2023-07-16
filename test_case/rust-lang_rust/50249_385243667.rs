plain
[00:50:11] .............................................................................ii.....................
[00:50:58] .........................................i....................................................i.ii..
[00:51:09] .............................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:51:37] .......................................................................
[00:51:53] ..iiiiiii...........................................................................................
[00:52:32] ....................................................................................................
[00:52:48] ..........................................................................
[00:52:48] test result: ok. 2955 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[00:52:48] 
---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:40] 
[01:22:40] running 182 tests
[01:23:22] ....................................................................................................
[01:24:13] ................................F................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
lvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-3.9/include -std=c++0x -gsplit-dwarf -Wl,-fuse-ld=gold -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -Werror=date-time -std=c++11 -ffunction-sections -fdata-sections -O2 -g -DNDEBUG  -fno-exceptions -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:25:27] 
[01:25:27] 
[01:25:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:25:27] Build completed unsuccessfully in 0:46:03
[01:25:27] Build completed unsuccessfully in 0:46:03
[01:25:27] Makefile:58: recipe for target 'check' failed
[01:25:27] make: *** [check] Error 1
obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123688 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0kd98txie-1uj3pes-6gvofocobazn
116684 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental
111752 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
111748 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
107428 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
107428 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102820 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102808 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm
102804 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm/s-f0keaopyz6-vmio0f-v7hez0f2m5v0
90976 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90976 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90972 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0ke8bnw02-1ixgbi3-1063n3gacp23l
89684 ./src/llvm/test/CodeGen
89616 ./obj/build/x86_64-unknown-linux-gnu/stage1
89592 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
86660 ./obj/build/x86_64-unknown-linux-gnu/doc/core
