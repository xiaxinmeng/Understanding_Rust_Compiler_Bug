plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 166 tests
...................................i............F.................F..............i.................. 100/166
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
................................i........F........................

---- [mir-opt] mir-opt/const_prop/switch_int.rs stdout ----
---- [mir-opt] mir-opt/const_prop/switch_int.rs stdout ----
thread '[mir-opt] mir-opt/const_prop/switch_int.rs' panicked at 'the mir dump file for switch_int.main.SimplifyBranches-after-const-prop.before.mir does not exist (requested in /checkout/src/test/mir-opt/const_prop/switch_int.rs)', src/tools/compiletest/src/runtest.rs:3371:17

---- [mir-opt] mir-opt/early_otherwise_branch_68867.rs stdout ----
---- [mir-opt] mir-opt/early_otherwise_branch_68867.rs stdout ----
thread '[mir-opt] mir-opt/early_otherwise_branch_68867.rs' panicked at 'the mir dump file for early_otherwise_branch_68867.try_sum.SimplifyBranches-final.after.mir does not exist (requested in /checkout/src/test/mir-opt/early_otherwise_branch_68867.rs)', src/tools/compiletest/src/runtest.rs:3371:17
---- [mir-opt] mir-opt/simplify_if.rs stdout ----
---- [mir-opt] mir-opt/simplify_if.rs stdout ----
thread '[mir-opt] mir-opt/simplify_if.rs' panicked at 'the mir dump file for simplify_if.main.SimplifyBranches-after-const-prop.before.mir does not exist (requested in /checkout/src/test/mir-opt/simplify_if.rs)', src/tools/compiletest/src/runtest.rs:3371:17

failures:
    [mir-opt] mir-opt/const_prop/switch_int.rs
    [mir-opt] mir-opt/early_otherwise_branch_68867.rs
    [mir-opt] mir-opt/early_otherwise_branch_68867.rs
    [mir-opt] mir-opt/simplify_if.rs

test result: FAILED. 160 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:18
