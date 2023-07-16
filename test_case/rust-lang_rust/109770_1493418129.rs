plain
---- [run-make] tests/run-make/compiler-lookup-paths stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/compiler-lookup-paths" && AR="ar" CC="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" CXX="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" NODE="/node-v15.14.0-linux-x64/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-wasm32-unknown-unknown" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="wasm32-unknown-unknown" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/compiler-lookup-paths/compiler-lookup-paths" "make"
--- stdout -------------------------------
clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown -v -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/compiler-lookup-paths/compiler-lookup-paths/libnative.o native.c
--- stderr -------------------------------
/bin/dash: 1: clang: not found
/bin/dash: 1: clang: not found
make: *** [../tools.mk:187: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/compiler-lookup-paths/compiler-lookup-paths/libnative.o] Error 127



---- [run-make] tests/run-make/hotplug_codegen_backend stdout ----
error: make failed
status: exit status: 2
status: exit status: 2
command: cd "/checkout/tests/run-make/hotplug_codegen_backend" && AR="ar" CC="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" CXX="clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfologicalview debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwarflinkerparallel dwp engine executionengine extensions filecheck frontendhlsl frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irprinter irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvtargetmca runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target targetparser textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" NODE="/node-v15.14.0-linux-x64/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-wasm32-unknown-unknown" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="wasm32-unknown-unknown" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/hotplug_codegen_backend/hotplug_codegen_backend" "make"
--- stdout -------------------------------
/bin/echo || exit 0 # This test requires /bin/echo to exist

LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/hotplug_codegen_backend/hotplug_codegen_backend:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/hotplug_codegen_backend/hotplug_codegen_backend -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/hotplug_codegen_backend/hotplug_codegen_backend  the_backend.rs --crate-name the_backend --crate-type dylib \
 -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/hotplug_codegen_backend/hotplug_codegen_backend/the_backend.dylib
--- stderr -------------------------------
error[E0463]: can't find crate for `rustc_codegen_ssa`
 --> the_backend.rs:4:1
  |
  |
4 | extern crate rustc_codegen_ssa;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_data_structures`
 --> the_backend.rs:5:1
  |
5 | extern crate rustc_data_structures;
5 | extern crate rustc_data_structures;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_driver`
 --> the_backend.rs:6:1
  |
6 | extern crate rustc_driver;
6 | extern crate rustc_driver;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_errors`
 --> the_backend.rs:7:1
  |
7 | extern crate rustc_errors;
7 | extern crate rustc_errors;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_hir`
 --> the_backend.rs:8:1
  |
8 | extern crate rustc_hir;
8 | extern crate rustc_hir;
  | ^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_metadata`
 --> the_backend.rs:9:1
  |
9 | extern crate rustc_metadata;
9 | extern crate rustc_metadata;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
  |
  = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_middle`
  --> the_backend.rs:10:1
   |
10 | extern crate rustc_middle;
10 | extern crate rustc_middle;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_session`
  --> the_backend.rs:11:1
   |
11 | extern crate rustc_session;
11 | extern crate rustc_session;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_span`
  --> the_backend.rs:12:1
   |
12 | extern crate rustc_span;
12 | extern crate rustc_span;
   | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_symbol_mangling`
  --> the_backend.rs:13:1
   |
13 | extern crate rustc_symbol_mangling;
13 | extern crate rustc_symbol_mangling;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
error[E0463]: can't find crate for `rustc_target`
  --> the_backend.rs:14:1
   |
14 | extern crate rustc_target;
14 | extern crate rustc_target;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
warning: ignoring --out-dir flag due to -o flag

error: aborting due to 11 previous errors; 1 warning emitted


For more information about this error, try `rustc --explain E0463`.
make: *** [Makefile:11: all] Error 1
------------------------------------------



failures:
    [run-make] tests/run-make/compiler-lookup-paths
    [run-make] tests/run-make/hotplug_codegen_backend
test result: FAILED. 126 passed; 2 failed; 181 ignored; 0 measured; 0 filtered out; finished in 14.21s

Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
Build completed unsuccessfully in 0:08:29
