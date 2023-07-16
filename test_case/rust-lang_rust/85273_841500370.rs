plain
.................................................................................................... 9500/11866
.................................................................................................... 9600/11866
.....................................................................i......i....................... 9700/11866
.................................................................................................... 9800/11866
..............iiiiiii..iiiiii.i..................................................................... 9900/11866
.................................................................................................... 10100/11866
.................................................................................................... 10200/11866
.................................................................................................... 10300/11866
.................................................................................................... 10400/11866
---
---- [ui] ui/async-await/async-unsafe-fn-call-in-safe.rs#thir stdout ----
diff of stderr:

14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
16 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/async-unsafe-fn-call-in-safe.rs:19:5
-    |
- LL |     S::f();
-    |     ^^^^^^ call to unsafe function
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/async-unsafe-fn-call-in-safe.rs:20:5
- LL |     f();
-    |     ^^^ call to unsafe function
-    |
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
- error: aborting due to 4 previous errors
+ error: aborting due to 2 previous errors
34 
35 For more information about this error, try `rustc --explain E0133`.
35 For more information about this error, try `rustc --explain E0133`.
36 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.thir/async-unsafe-fn-call-in-safe.thir.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/async-unsafe-fn-call-in-safe.rs`

error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-unsafe-fn-call-in-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-unsafe-fn-call-in-safe.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     S::f(); //~ ERROR call to unsafe function is unsafe
   |     ^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     f(); //~ ERROR call to unsafe function is unsafe
   |     ^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0133`.


------------------------------------------


---- [ui] ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs#thir stdout ----
diff of stderr:

1 error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/const-extern-fn-requires-unsafe.rs:11:5
- LL |     foo();
-    |     ^^^^^ call to unsafe function
-    |
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
- 
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
11    |
11    |
12 LL |     let a: [u8; foo()];
14    |
14    |
15    = note: consult the function's documentation for information on how to avoid undefined behavior
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
18 
19 For more information about this error, try `rustc --explain E0133`.
19 For more information about this error, try `rustc --explain E0133`.
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.thir/const-extern-fn-requires-unsafe.thir.stderr
To only update this specific test, also pass `--test-args consts/const-extern-fn/const-extern-fn-requires-unsafe.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thir" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.thir" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-requires-unsafe.thir/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
   |
   |
LL |     let a: [u8; foo()];
   |                 ^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
error: aborting due to previous error

For more information about this error, try `rustc --explain E0133`.

---
test result: FAILED. 11768 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 119.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:11
