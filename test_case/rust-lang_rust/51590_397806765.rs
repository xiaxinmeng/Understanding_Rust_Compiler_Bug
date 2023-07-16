plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:37] 
[01:23:37] running 186 tests
[01:24:19] ....................i......................................F........................................
[01:25:10] .....................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
6_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend  some_crate.rs --crate-name some_crate --crate-type bin -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/some_crate \
[01:26:24]  -Z codegen-backend=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib -Z unstable-options
[01:26:24] Makefile:4: recipe for target 'all' failed
[01:26:24] make[1]: Leaving directory '/checkout/src/test/run-make-fulldeps/hotplug_codegen_backend'
[01:26:24] ------------------------------------------
[01:26:24] stderr:
[01:26:24] ------------------------------------------
[01:26:24] ------------------------------------------
[01:26:24] warning: ignoring --out-dir flag due to -o flag
[01:26:24] 
[01:26:24] warning: ignoring --out-dir flag due to -o flag
[01:26:24] error[E0463]: can't find crate for `std`
[01:26:24] 
[01:26:24] error: aborting due to previous error
[01:26:24] 
[01:26:24] 
[01:26:24] For more information about this error, try `rustc --explain E0463`.
[01:26:24] make[1]: *** [all] Error 101
[01:26:24] ------------------------------------------
[01:26:24] 
[01:26:24] 
[01:26:24] thre" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo asmparser asmprinter bitreader bitwriter bpf bpfasmprinter bpfcodegen bpfdesc bpfinfo codegen core coverage debuginfocodeview debuginfodwarf debuginfopdb engine executionengine globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinte4682416 .
3405216 ./obj/build
2641796 ./obj/build/x86_64-unknown-linux-gnu
810212 ./obj/build/x86_64-unknown-linux-gnu/test
728792 ./src
---
149120 ./src/llvm-emscripten/test
146272 ./.git/modules
146268 ./.git/modules/src
123372 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123368 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f21b4ybsdm-1d7w2n-34nhbs5zu1g0h
