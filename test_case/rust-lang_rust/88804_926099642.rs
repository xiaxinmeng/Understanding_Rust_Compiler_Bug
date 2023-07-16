plain
test [rustdoc] rustdoc\where-sized.rs ... ok
test [rustdoc] rustdoc\without-redirect.rs ... ok
test [rustdoc] rustdoc\wrapping.rs ... ok
test [rustdoc] rustdoc\where.rs ... ok
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=i686-pc-windows-msvc target=i686-pc-windows-msvc
failures:

---- [rustdoc] rustdoc\macro-document-private-duplicate.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "C:\\hostedtoolcache\\windows\\Python\\3.9.7\\x64\\python3.exe" "D:\\a\\rust\\rust\\src/etc/htmldocck.py" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc\\macro-document-private-duplicate" "D:\\a\\rust\\rust\\src/test\\rustdoc\\macro-document-private-duplicate.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `PATTERN` did not match
 // @has macro_document_private_duplicate/macro.a_macro.html 'Doc 1.'
19: @!has check failed
 `PATTERN` did not match
 // @!has macro_document_private_duplicate/macro.a_macro.html 'Doc 2.'
Encountered 2 errors

------------------------------------------

---
test result: FAILED. 462 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out; finished in 87.18s



command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage0-tools-bin\\compiletest.exe" "--compile-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin" "--run-lib-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "--rustc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustc.exe" "--rustdoc-path" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\stage2\\bin\\rustdoc.exe" "--src-base" "D:\\a\\rust\\rust\\src/test\\rustdoc" "--build-base" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\test\\rustdoc" "--stage-id" "stage2-i686-pc-windows-msvc" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "i686-pc-windows-msvc" "--host" "i686-pc-windows-msvc" "--llvm-filecheck" "D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\llvm\\build\\bin\\FileCheck.exe" "--nodejs" "C:\\Program Files\\nodejs\\node" "--npm" "C:\\Program Files\\nodejs\\npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=D:\\a\\rust\\rust\\build\\i686-pc-windows-msvc\\native\\rust-test-helpers" "--docck-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.7\\x64\\python3.exe" "--lldb-python" "C:\\hostedtoolcache\\windows\\Python\\3.9.7\\x64\\python3.exe" "--gdb" "C:\\msys64\\usr\\bin\\gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:41:05
Build completed unsuccessfully in 0:41:05
make: *** [Makefile:72: ci-subset-1] Error 1
