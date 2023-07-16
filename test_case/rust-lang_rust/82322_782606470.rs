plain
.................................................................................................... 9200/11474
.................................................................................................... 9300/11474
.................................................................................................... 9400/11474
.............................i......i............................................................... 9500/11474
....................................................................iiiiiii..iiiiii.i............... 9600/11474
.................................................................................................... 9800/11474
.................................................................................................... 9900/11474
.................................................................................................... 10000/11474
.................................................................................................... 10100/11474
---

---- [ui] ui/option-to-result.rs stdout ----
diff of stderr:

- error[E0277]: `?` couldn't convert the error to `()`
+ error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
3    |
3    |
- LL | fn test_result() -> Result<(),()> {
-    |                     ------------- expected `()` because of this
- LL |     let a:Option<()> = Some(());
- LL |     a?;
-    |     ^^ the trait `From<result::sadness::PleaseCallTheOkOrMethodToUseQuestionMarkOnOptionsInAMethodThatReturnsResult>` is not implemented for `()`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | / fn test_result() -> Result<(),()> {
+ LL | |     let a:Option<()> = Some(());
+ LL | |     a?;
+    | |     ^^ cannot use the `?` operator in a function that returns `Result<(), ()>`
+ LL | |     Ok(())
+ LL | | }
+    | |_- this function should return `Result` or `Option` to accept `?`
9    |
-    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
-    = note: required because of the requirements on the impl of `FromResidual<Option<!>>` for `Result<(), ()>`
+    = help: the trait `FromResidual<Option<!>>` is not implemented for `Result<(), ()>`
12    = note: required by `from_residual`
13 
14 error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/option-to-result/option-to-result.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/option-to-result/option-to-result.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args option-to-result.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/option-to-result.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/option-to-result" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/option-to-result/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  --> /checkout/src/test/ui/option-to-result.rs:5:5
   |
LL | / fn test_result() -> Result<(),()> {
LL | |     let a:Option<()> = Some(());
LL | |     a?;//~ ERROR `?` couldn't convert the error to `()`
   | |     ^^ cannot use the `?` operator in a function that returns `Result<(), ()>`
LL | |     Ok(())
LL | | }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Option<!>>` is not implemented for `Result<(), ()>`
   = note: required by `from_residual`

error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  --> /checkout/src/test/ui/option-to-result.rs:11:5
   |
LL | / fn test_option() -> Option<i32>{
LL | |     let a:Result<i32, i32> = Ok(5);
LL | |     a?;//~ ERROR the `?` operator can only be used in a function that returns `Result` or `Option`
   | |     ^^ cannot use the `?` operator in a function that returns `Option<i32>`
LL | |     Some(5)
LL | | }
   | |_- this function should return `Result` or `Option` to accept `?`
   |
   = help: the trait `FromResidual<Result<!, i32>>` is not implemented for `Option<i32>`
   = note: required by `from_residual`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 11380 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 140.55s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:24
