plain
............................i.ii......ii...i...........................iiiiii...................
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] codegen/intrinsics/const_eval_select.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/intrinsics/const_eval_select.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/const_eval_select" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/const_eval_select/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0631]: type mismatch in function arguments
    --> /checkout/src/test/codegen/intrinsics/const_eval_select.rs:16:35
     |
8    | const fn foo(_: (i32,)) -> i32 { 1 }
     | ------------------------------ found signature of `fn((i32,)) -> _`
...
16   |     const_eval_select((42,), foo, hi);
     |     -----------------             ^^ expected signature of `fn({integer}) -> _`
     |     required by a bound introduced by this call
     |
note: required by a bound in `const_eval_select`
    --> /checkout/library/core/src/intrinsics.rs:2270:8
    --> /checkout/library/core/src/intrinsics.rs:2270:8
     |
2270 |     F: ~const FnOnce<ARG, Output = RET>,
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
error[E0631]: type mismatch in function arguments
    --> /checkout/src/test/codegen/intrinsics/const_eval_select.rs:16:35
     |
     |
11   | pub fn hi((n,): (i32,)) -> i32 { n }
     | ------------------------------ found signature of `fn((i32,)) -> _`
...
16   |     const_eval_select((42,), foo, hi);
     |     -----------------             ^^ expected signature of `fn({integer}) -> _`
     |     required by a bound introduced by this call
     |
note: required by a bound in `const_eval_select`
    --> /checkout/library/core/src/intrinsics.rs:2271:8
    --> /checkout/library/core/src/intrinsics.rs:2271:8
     |
2271 |     G: FnOnce<ARG, Output = RET> + ~const Drop,
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0631`.

---
test result: FAILED. 232 passed; 1 failed; 63 ignored; 0 measured; 0 filtered out; finished in 3.12s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:08
