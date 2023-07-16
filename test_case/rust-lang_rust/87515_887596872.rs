plain
.......................................................................ii........................... 12100/12180
................................................................................
failures:

---- [ui] ui/traits/trait-upcasting/replace-vptr.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 11 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/replace-vptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/replace-vptr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/replace-vptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x60b793)[0x7fdcc65fa793]
/lib/x86_64-linux-gnu/libc.so.6(+0x3f040)[0x7fdcc58b3040]
/usr/lib/x86_64-linux-gnu/libLLVM-10.so.1(_ZN4llvm11PointerType3getEPNS_4TypeEj+0x12)[0x7fdcc1c3eda2]
/usr/lib/x86_64-linux-gnu/libLLVM-10.so.1(+0x9cef5a)[0x7fdcc1b50f5a]
/usr/lib/x86_64-linux-gnu/libLLVM-10.so.1(+0x9ee66c)[0x7fdcc1b7066c]
/usr/lib/x86_64-linux-gnu/libLLVM-10.so.1(+0xa00be4)[0x7fdcc1b82be4]
/usr/lib/x86_64-linux-gnu/libLLVM-10.so.1(LLVMBuildStructGEP+0x49)[0x7fdcc1b80669]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x86ffd6)[0x7fdcc685efd6]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x934a1d)[0x7fdcc6923a1d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x927e3e)[0x7fdcc6916e3e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x872e16)[0x7fdcc6861e16]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x8b8aa3)[0x7fdcc68a7aa3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x904155)[0x7fdcc68f3155]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x839ea1)[0x7fdcc6828ea1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x903bb2)[0x7fdcc68f2bb2]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x871e11)[0x7fdcc6860e11]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(_ZN109_$LT$rustc_codegen_llvm..LlvmCodegenBackend$u20$as$u20$rustc_codegen_ssa..traits..backend..CodegenBackend$GT$13codegen_crate17h69d6935174cd2c1aE+0x69)[0x7fdcc689cdd9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x7add01)[0x7fdcc679cd01]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x7462ba)[0x7fdcc67352ba]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(_ZN15rustc_interface7queries7Queries15ongoing_codegen17h5aa42a039abe4eb4E+0xb8)[0x7fdcc6728988]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x65266b)[0x7fdcc664166b]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x60dbbc)[0x7fdcc65fcbbc]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x653e7c)[0x7fdcc6642e7c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x649ea3)[0x7fdcc6638ea3]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x60fa2f)[0x7fdcc65fea2f]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x67a496)[0x7fdcc6669496]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/librustc_driver-4c5decbaebf19c76.so(+0x6081ea)[0x7fdcc65f71ea]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/libstd-acc102086c90e281.so(rust_metadata_std_21736b398a6b3262+0xaafaa)[0x7fdcc5d0ffaa]
/lib/x86_64-linux-gnu/libpthread.so.0(+0x76db)[0x7fdcc09c96db]
/lib/x86_64-linux-gnu/libc.so.6(clone+0x3f)[0x7fdcc599571f]
------------------------------------------




failures:
    [ui] ui/traits/trait-upcasting/replace-vptr.rs
test result: FAILED. 12076 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 127.76s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:09
