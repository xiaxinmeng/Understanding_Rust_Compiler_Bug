plain
---- [run-make] tests/run-make/wasm-abi stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/wasm-abi" && AR="ar" CC="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" CXX="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker loongarch loongarchasmparser loongarchcodegen loongarchdesc loongarchdisassembler loongarchinfo lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" NODE="/node-v15.14.0-linux-x64/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-wasm32-unknown-unknown" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="wasm32-unknown-unknown" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-abi/wasm-abi" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-abi/wasm-abi:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-abi/wasm-abi -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-abi/wasm-abi  foo.rs --target wasm32-unknown-unknown
/node-v15.14.0-linux-x64/bin/node foo.js /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-abi/wasm-abi/foo.wasm
--- stderr -------------------------------
/checkout/tests/run-make/wasm-abi/foo.js:6
/checkout/tests/run-make/wasm-abi/foo.js:6
const m = new WebAssembly.Module(buffer);


CompileError: WebAssembly.Module(): Compiling function #136:"_ZN4core3num7flt2dec8strategy6dragon9mul_pow101..." failed: expected 1 elements on the stack for fallthru to @11, found 0 @+22373
    at Object.<anonymous> (/checkout/tests/run-make/wasm-abi/foo.js:6:11)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
Build completed unsuccessfully in 0:16:43
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47
make: *** [Makefile:7: all] Error 1


---- [run-make] tests/run-make/wasm-export-all-symbols stdout ----


error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/wasm-export-all-symbols" && AR="ar" CC="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" CXX="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker loongarch loongarchasmparser loongarchcodegen loongarchdesc loongarchdisassembler loongarchinfo lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" NODE="/node-v15.14.0-linux-x64/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-wasm32-unknown-unknown" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="wasm32-unknown-unknown" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols  bar.rs --target wasm32-unknown-unknown
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols  foo.rs --target wasm32-unknown-unknown
/node-v15.14.0-linux-x64/bin/node verify.js /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols/foo.wasm
exports [
  { name: 'memory', kind: 'memory' },
  { name: 'foo', kind: 'function' },
  { name: 'FOO', kind: 'global' },
  { name: '__data_end', kind: 'global' },
  { name: '__heap_base', kind: 'global' }
]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols  main.rs --target wasm32-unknown-unknown
/node-v15.14.0-linux-x64/bin/node verify.js /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-export-all-symbols/wasm-export-all-symbols/main.wasm
--- stderr -------------------------------
/checkout/tests/run-make/wasm-export-all-symbols/verify.js:6
/checkout/tests/run-make/wasm-export-all-symbols/verify.js:6
let m = new WebAssembly.Module(buffer);


CompileError: WebAssembly.Module(): Compiling function #35:"_ZN3std2rt19lang_start_internal17h6d5b51228aef7..." failed: expected 1 elements on the stack for fallthru to @5, found 0 @+12243
    at Object.<anonymous> (/checkout/tests/run-make/wasm-export-all-symbols/verify.js:6:9)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47
make: *** [Makefile:10: all] Error 1


---- [run-make] tests/run-make/wasm-symbols-not-exported stdout ----


error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/wasm-symbols-not-exported" && AR="ar" CC="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" CXX="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker loongarch loongarchasmparser loongarchcodegen loongarchdesc loongarchdisassembler loongarchinfo lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" NODE="/node-v15.14.0-linux-x64/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-wasm32-unknown-unknown" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="wasm32-unknown-unknown" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-symbols-not-exported/wasm-symbols-not-exported" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-symbols-not-exported/wasm-symbols-not-exported:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-symbols-not-exported/wasm-symbols-not-exported -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-symbols-not-exported/wasm-symbols-not-exported  foo.rs --target wasm32-unknown-unknown
/node-v15.14.0-linux-x64/bin/node verify-exported-symbols.js /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/wasm-symbols-not-exported/wasm-symbols-not-exported/foo.wasm
--- stderr -------------------------------
/checkout/tests/run-make/wasm-symbols-not-exported/verify-exported-symbols.js:6
/checkout/tests/run-make/wasm-symbols-not-exported/verify-exported-symbols.js:6
let m = new WebAssembly.Module(buffer);


CompileError: WebAssembly.Module(): Compiling function #56:"_ZN60_$LT$std..io..error..Error$u20$as$u20$core..." failed: expected 1 elements on the stack for fallthru to @3, found 0 @+15832
    at Object.<anonymous> (/checkout/tests/run-make/wasm-symbols-not-exported/verify-exported-symbols.js:6:9)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47
make: *** [Makefile:7: all] Error 1



failures:
