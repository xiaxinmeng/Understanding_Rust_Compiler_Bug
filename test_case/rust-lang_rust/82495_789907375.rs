plain
failures:

---- [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-74564-if-expr-stack-overflow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-74564-if-expr-stack-overflow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-74564-if-expr-stack-overflow/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------

thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
------------------------------------------


---- [ui] ui/nll/issue-54382-use-span-of-tail-of-block.rs stdout ----
---- [ui] ui/nll/issue-54382-use-span-of-tail-of-block.rs stdout ----
diff of stderr:

1 error[E0597]: `_thing1` does not live long enough
3    |
3    |
- LL |             D("other").next(&_thing1)
-    |             |               |
-    |             |               |
-    |             |               borrowed value does not live long enough
-    |             a temporary with access to the borrow is created here ...
- LL |     }
- LL |     }
-    |     - `_thing1` dropped here while still borrowed
+ LL | /         {
+ LL | |             let _thing2 = D("thing2");
+ LL | |             side_effects();
+ LL | |             D("other").next(&_thing1)
+    | |                             ^^^^^^^^ borrowed value does not live long enough
+ LL | |
+ LL | |         }
+    | |_________- a temporary with access to the borrow is created here ...
+ LL |       }
+    |       - `_thing1` dropped here while still borrowed
- LL |     ;
- LL |     ;
-    |     - ... and the borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `D`
+ LL |       ;
+    |       - ... and the borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `D`
15    |
16 help: consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped


- LL |             D("other").next(&_thing1);
+ LL |         };
+    |          ^
20 
21 error: aborting due to previous error
21 error: aborting due to previous error
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-54382-use-span-of-tail-of-block/issue-54382-use-span-of-tail-of-block.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-54382-use-span-of-tail-of-block.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-54382-use-span-of-tail-of-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-54382-use-span-of-tail-of-block" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-54382-use-span-of-tail-of-block/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `_thing1` does not live long enough
   |
LL | /         {
LL | /         {
LL | |             let _thing2 = D("thing2");
LL | |             side_effects();
LL | |             D("other").next(&_thing1)
   | |                             ^^^^^^^^ borrowed value does not live long enough
LL | | //~^ ERROR does not live long enough
LL | |         }
   | |_________- a temporary with access to the borrow is created here ...
LL |       }
   |       - `_thing1` dropped here while still borrowed
LL |       ;
LL |       ;
   |       - ... and the borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `D`
   |
help: consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped
LL |         };
   |          ^

error: aborting due to previous error
---
test result: FAILED. 11429 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 131.45s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:02
