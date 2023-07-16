plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.103 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 128 tests
...........................................F....F...........................................F....... 100/128
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] incremental/ich_nested_items.rs stdout ----


error in revision `cfail1`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_nested_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items/ich_nested_items.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_nested_items/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL | #[rustc_clean(label = "hir_owner", cfg = "cfail2")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL | #[rustc_dirty(label = "hir_owner_nodes", cfg = "cfail2")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_dirty(label = "hir_owner", cfg = "cfail2")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_clean(label = "hir_owner_nodes", cfg = "cfail2")]

error: aborting due to 4 previous errors



------------------------------------------


---- [incremental] incremental/ich_resolve_results.rs stdout ----

error in revision `rpass1`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_resolve_results.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/ich_resolve_results.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_clean(label="hir_owner", cfg="rpass2")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_clean(label="hir_owner_nodes", cfg="rpass2")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_clean(label="hir_owner", cfg="rpass3")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_dirty(label="hir_owner_nodes", cfg="rpass3")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_clean(label="hir_owner", cfg="rpass2")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_clean(label="hir_owner_nodes", cfg="rpass2")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_clean(label="hir_owner", cfg="rpass3")]


error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL |     #[rustc_dirty(label="hir_owner_nodes", cfg="rpass3")]

error: aborting due to 8 previous errors



------------------------------------------


---- [incremental] incremental/spans_significant_w_panic.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/spans_significant_w_panic.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: attribute requires -Z query-dep-graph to be enabled
   |
   |
LL | #[rustc_dirty(label="optimized_mir", cfg="rpass2")]

error: aborting due to previous error


---
test result: FAILED. 125 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.28s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:53
