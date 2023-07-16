plain

---- [ui] ui/mir/issue-80742.rs stdout ----
diff of stderr:

12 LL |     [u8; size_of::<T>() + 1]: ,
13    |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at $DIR/issue-80742.rs:23:10
14 
- error[E0599]: no function or associated item named `new` found for struct `Inline<dyn Debug>` in the current scope
+ error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
17    |
17    |
18 LL | / struct Inline<T>

25    | |_- function or associated item `new` not found for this
26 ...
27 LL |       let dst = Inline::<dyn Debug>::new(0);
-    |                                      ^^^ function or associated item not found in `Inline<dyn Debug>`
+    |                                      ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
29    | 
30   ::: $SRC_DIR/core/src/fmt/mod.rs:LL:COL

32 LL |   pub trait Debug {
32 LL |   pub trait Debug {
33    |   --------------- doesn't satisfy `dyn Debug: Sized`
34    |
-    = note: the method `new` exists but the following trait bounds were not satisfied:
+    = note: the following trait bounds were not satisfied:
36            `dyn Debug: Sized`
38 error[E0080]: evaluation of constant value failed


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-80742/issue-80742.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mir/issue-80742.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-80742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-80742" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-80742/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/mem/mod.rs:310:5
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     size_of called on unsized type `dyn Debug`
   |     inside `std::mem::size_of::<dyn Debug>` at /checkout/library/core/src/mem/mod.rs:310:5
  ::: /checkout/src/test/ui/mir/issue-80742.rs:23:10
   |
   |
LL |     [u8; size_of::<T>() + 1]: ,
   |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at /checkout/src/test/ui/mir/issue-80742.rs:23:10

error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
   |
   |
LL | / struct Inline<T>
LL | | where
LL | |     [u8; size_of::<T>() + 1]: ,
LL | | {
LL | |     _phantom: PhantomData<T>,
LL | |     buf: [u8; size_of::<T>() + 1],
LL | | }
   | |_- function or associated item `new` not found for this
...
LL |       let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |                                      ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/fmt/mod.rs:554:1
   |
LL |   pub trait Debug {
LL |   pub trait Debug {
   |   --------------- doesn't satisfy `dyn Debug: Sized`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `dyn Debug: Sized`
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/mem/mod.rs:310:5
   |
LL |     intrinsics::size_of::<T>()
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     size_of called on unsized type `dyn Debug`
   |     inside `std::mem::size_of::<dyn Debug>` at /checkout/library/core/src/mem/mod.rs:310:5
  ::: /checkout/src/test/ui/mir/issue-80742.rs:15:10
   |
   |
LL |     [u8; size_of::<T>() + 1]: ,
   |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at /checkout/src/test/ui/mir/issue-80742.rs:15:10

error[E0277]: the size for values of type `dyn Debug` cannot be known at compilation time
   |
   |
LL | struct Inline<T>
   |               - required by this bound in `Inline`
...
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |               ^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `dyn Debug`
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Inline<T: ?Sized>

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0080, E0277, E0599.
---
test result: FAILED. 11213 passed; 1 failed; 88 ignored; 0 measured; 0 filtered out; finished in 135.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:06
