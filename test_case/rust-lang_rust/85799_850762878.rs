plain
..............................................i......................................i.............. 10700/11934
.................................................................................................... 10800/11934
.................................................................................................... 10900/11934
.................................................................................................... 11000/11934
.....................................................................F.F............................ 11100/11934
..................ii................................................................................ 11300/11934
.................................................................................................... 11400/11934
.................................................................................................... 11500/11934
.................................................................................................... 11600/11934
---

18    | 
19   ::: $SRC_DIR/core/src/result.rs:LL:COL
20    |
- LL |     type Ok = T;
-    |          -- type mismatch with `&str` here
+ LL |     type Output = T;
+    |          ------ type mismatch with `&str` here
23 
24 error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Output == ()`
25   --> $DIR/try-block-bad-type.rs:15:39
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
29    | 
30   ::: $SRC_DIR/core/src/result.rs:LL:COL
31    |
31    |
- LL |     type Ok = T;
-    |          -- type mismatch with `()` here
+ LL |     type Output = T;
+    |          ------ type mismatch with `()` here
34 
35 error[E0277]: a `try` block must return `Result` or `Option` (or another type that implements `Try`)
36   --> $DIR/try-block-bad-type.rs:17:25

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `?` couldn't convert the error to `TryFromSliceError`
   |
   |
LL |         Err("")?; //~ ERROR `?` couldn't convert the error
   |                ^ the trait `From<&str>` is not implemented for `TryFromSliceError`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <TryFromSliceError as From<Infallible>>
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, &str>>` for `Result<u32, TryFromSliceError>`
   = note: required by `from_residual`

error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Output == &str`
   |
   |
LL |         "" //~ ERROR type mismatch
   |         ^^ expected `i32`, found `&str`
  ::: /checkout/library/core/src/result.rs:1653:10
   |
LL |     type Output = T;
LL |     type Output = T;
   |          ------ type mismatch with `&str` here

error[E0271]: type mismatch resolving `<Result<i32, i32> as Try>::Output == ()`
   |
   |
LL |     let res: Result<i32, i32> = try { }; //~ ERROR type mismatch
   |                                       ^ expected `i32`, found `()`
  ::: /checkout/library/core/src/result.rs:1653:10
   |
LL |     type Output = T;
LL |     type Output = T;
   |          ------ type mismatch with `()` here

error[E0277]: a `try` block must return `Result` or `Option` (or another type that implements `Try`)
   |
   |
LL |     let res: () = try { };
   |                         ^ could not wrap the final value of the block as `()` doesn't implement `Try`
   |
   = help: the trait `Try` is not implemented for `()`
   = note: required by `from_output`

error[E0277]: a `try` block must return `Result` or `Option` (or another type that implements `Try`)
   |
   |
LL |     let res: i32 = try { 5 }; //~ ERROR a `try` block must return `Result` or `Option`
   |                          ^ could not wrap the final value of the block as `i32` doesn't implement `Try`
   = help: the trait `Try` is not implemented for `i32`
   = note: required by `from_output`

error: aborting due to 5 previous errors
---

9    | 
10   ::: $SRC_DIR/core/src/option.rs:LL:COL
11    |
- LL |     type Ok = T;
-    |          -- type mismatch with `{integer}` here
+ LL |     type Output = T;
+    |          ------ type mismatch with `{integer}` here
14 
15 error[E0271]: type mismatch resolving `<Option<i32> as Try>::Output == ()`
16   --> $DIR/try-block-type-error.rs:16:5
20    | 
21   ::: $SRC_DIR/core/src/option.rs:LL:COL
22    |
22    |
- LL |     type Ok = T;
-    |          -- type mismatch with `()` here
+ LL |     type Output = T;
+    |          ------ type mismatch with `()` here
26 error: aborting due to 2 previous errors
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-type-error/try-block-type-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-block/try-block-type-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-type-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-type-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Option<f32> as Try>::Output == {integer}`
   |
LL |         42
   |         ^^
   |         |
   |         |
   |         expected `f32`, found integer
   |         help: use a float literal: `42.0`
  ::: /checkout/library/core/src/option.rs:1671:10
   |
LL |     type Output = T;
LL |     type Output = T;
   |          ------ type mismatch with `{integer}` here

error[E0271]: type mismatch resolving `<Option<i32> as Try>::Output == ()`
   |
LL |     };
LL |     };
   |     ^ expected `i32`, found `()`
  ::: /checkout/library/core/src/option.rs:1671:10
   |
LL |     type Output = T;
LL |     type Output = T;
   |          ------ type mismatch with `()` here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.

---
test result: FAILED. 11835 passed; 2 failed; 97 ignored; 0 measured; 0 filtered out; finished in 117.94s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:41
