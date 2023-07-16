plain
.................................................................................................... 7100/11825
.................................................................................................... 7200/11825
.................................................................................................... 7300/11825
....ii................i..i..ii...................................................................... 7400/11825
.......................F....................F....................................................... 7500/11825
.................................................................................................... 7700/11825
........................................................i..ii....................................... 7800/11825
.......................ii........................................................................... 7900/11825
.................................................................................................... 8000/11825
---
.................................................................................................... 9400/11825
.................................................................................................... 9500/11825
.................................................................................................... 9600/11825
.............................................i......i............................................... 9700/11825
...........................................................................................iiiiiii.. 9800/11825
iiiiii.i............................................................................................ 9900/11825
.................................................................................................... 10100/11825
.................................................................................................... 10200/11825
.................................................................................................... 10300/11825
.................................................................................................... 10400/11825
---

---- [ui] ui/binding/empty-types-in-patterns.rs stdout ----
diff of stderr:

- warning: the feature `never_type_fallback` has been stable since 1.49.0 and no longer requires an attribute to enable
-   --> $DIR/empty-types-in-patterns.rs:3:12
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | #![feature(never_type_fallback)]
-    |
-    = note: `#[warn(stable_features)]` on by default
- 
- warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args binding/empty-types-in-patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/empty-types-in-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/empty-types-in-patterns/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/empty-types-in-patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/coercion/coerce-issue-49593-box-never.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-issue-49593-box-never.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-issue-49593-box-never/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `(): std::error::Error` is not satisfied
   |
   |
LL |     /* *mut $0 is coerced to Box<dyn Error> here */ Box::<_ /* ! */>::new(x)
   |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::error::Error` is not implemented for `()`
   = note: required for the cast to the object type `dyn std::error::Error`

error: aborting due to previous error

---

---- [ui] ui/never_type/diverging-fallback-control-flow.rs stdout ----
diff of stderr:

- warning: the feature `never_type_fallback` has been stable since 1.49.0 and no longer requires an attribute to enable
-   --> $DIR/diverging-fallback-control-flow.rs:11:12
-    |
- LL | #![feature(never_type_fallback)]
-    |
-    = note: `#[warn(stable_features)]` on by default
- 
- warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args never_type/diverging-fallback-control-flow.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/diverging-fallback-control-flow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-fallback-control-flow/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-fallback-control-flow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/never_type/never-value-fallback-issue-66757.rs stdout ----
diff of stderr:

- warning: the feature `never_type_fallback` has been stable since 1.49.0 and no longer requires an attribute to enable
-   --> $DIR/never-value-fallback-issue-66757.rs:12:12
-    |
- LL | #![feature(never_type_fallback)]
-    |
-    = note: `#[warn(stable_features)]` on by default
- 
- warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args never_type/never-value-fallback-issue-66757.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/never-value-fallback-issue-66757.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-value-fallback-issue-66757/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/never-value-fallback-issue-66757/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/pattern/usefulness/uninhabited.rs stdout ----
diff of stderr:

- warning: the feature `never_type_fallback` has been stable since 1.49.0 and no longer requires an attribute to enable
-   --> $DIR/uninhabited.rs:6:12
-    |
- LL | #![feature(never_type_fallback)]
-    |
-    = note: `#[warn(stable_features)]` on by default
- 
- warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args pattern/usefulness/uninhabited.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/uninhabited.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/uninhabited" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/uninhabited/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 11725 passed; 5 failed; 95 ignored; 0 measured; 0 filtered out; finished in 120.19s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:57
