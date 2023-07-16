plain
---- [mir-opt] mir-opt/coverage_graphviz.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/coverage_graphviz.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "-Copt-level=1" "-Zdump-mir=all" "-Zmir-opt-level=4" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/coverage_graphviz" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/coverage_graphviz" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "instrument-coverage" "-Z" "dump-mir-graphviz" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/coverage_graphviz/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Hashing HIR bodies is forbidden.', compiler/rustc_query_system/src/ich/hcx.rs:108:40

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (91a276722 2021-10-13) running on aarch64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=4 -Z validate-mir -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/coverage_graphviz -Z instrument-coverage -Z dump-mir-graphviz -C opt-level=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_promoted] processing `main`
#1 [mir_borrowck] borrow-checking `main`

------------------------------------------



---- [mir-opt] mir-opt/instrument_coverage.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/instrument_coverage.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "-Copt-level=1" "-Zdump-mir=all" "-Zmir-opt-level=4" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/instrument_coverage" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/instrument_coverage" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "instrument-coverage" "--remap-path-prefix=/checkout/src/test/mir-opt=/the/src" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/instrument_coverage/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Hashing HIR bodies is forbidden.', compiler/rustc_query_system/src/ich/hcx.rs:108:40

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (91a276722 2021-10-13) running on aarch64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z dump-mir=all -Z mir-opt-level=4 -Z validate-mir -Z dump-mir-exclude-pass-number -Z dump-mir-dir=/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt/instrument_coverage -Z instrument-coverage -C opt-level=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_promoted] processing `main`
#1 [mir_borrowck] borrow-checking `main`

------------------------------------------


---
test result: FAILED. 161 passed; 2 failed; 2 ignored; 0 measured; 0 filtered out; finished in 2.06s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:55
