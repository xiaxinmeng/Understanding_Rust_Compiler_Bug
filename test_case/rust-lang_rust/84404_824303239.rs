plain
......................F............................................................................. 9400/11763
.................................................................................................... 9500/11763
............................................................................................i......i 9600/11763
.................................................................................................... 9700/11763
......................................iiiiiii..iiiiii.i............................................. 9800/11763
.................................................................................................... 10000/11763
.................................................................................................... 10100/11763
.................................................................................................... 10200/11763
.................................................................................................... 10300/11763
---

---- [ui] ui/reify-intrinsic.rs stdout ----
diff of stderr:

19 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;
21 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0308]: cannot coerce intrinsics to function pointers
+   --> $DIR/reify-intrinsic.rs:18:9
+ LL |         std::intrinsics::copy::<i32>,
+ LL |         std::intrinsics::copy::<i32>,
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot coerce intrinsics to function pointers
+    |
+    = note: expected type `unsafe extern "rust-intrinsic" fn(_, _, _) {copy_nonoverlapping::<i32>}`
+            found fn item `unsafe extern "rust-intrinsic" fn(_, _, _) {std::intrinsics::copy::<i32>}`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error: aborting due to 3 previous errors
23 
24 Some errors have detailed explanations: E0308, E0606.
24 Some errors have detailed explanations: E0308, E0606.
25 For more information about an error, try `rustc --explain E0308`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args reify-intrinsic.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: cannot coerce intrinsics to function pointers
   |
   |
LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
   |            -------------------------------------------------   ^^^^^^^^^^^^^^^^^^^ cannot coerce intrinsics to function pointers
   |            expected due to this
   |
   |
   = note: expected fn pointer `unsafe extern "rust-intrinsic" fn(isize) -> usize`
                 found fn item `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
help: use parentheses to call this function
   |
LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute(...);


error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
   |
   |
LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;


error[E0308]: cannot coerce intrinsics to function pointers
   |
LL |         std::intrinsics::copy::<i32>,
LL |         std::intrinsics::copy::<i32>,
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot coerce intrinsics to function pointers
   |
   = note: expected type `unsafe extern "rust-intrinsic" fn(_, _, _) {copy_nonoverlapping::<i32>}`
           found fn item `unsafe extern "rust-intrinsic" fn(_, _, _) {std::intrinsics::copy::<i32>}`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0606.
For more information about an error, try `rustc --explain E0308`.
---
test result: FAILED. 11665 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 123.98s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:42
