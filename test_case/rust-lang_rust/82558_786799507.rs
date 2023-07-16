plain
.................................................................................................... 9200/11506
.................................................................................................... 9300/11506
.................................................................................................... 9400/11506
.....................................................i......i....................................... 9500/11506
............................................................................................iiiiiii. 9600/11506
.iiiiii.i........................................................................................... 9700/11506
.................................................................................................... 9900/11506
.................................................................................................... 10000/11506
.................................................................................................... 10100/11506
.................................................................................................... 10200/11506
---
.................................................................................................... 11500/11506
......
failures:

---- [ui] ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs stdout ----


17    |                ^^^^^^^^^^^^^^^^^^^^^ the trait `From<&A>` is not implemented for `&'static B`
18    |
19    = note: required because of the requirements on the impl of `Into<&'static B>` for `&A`
+ help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
+    |
+ LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) where &'static B: From<&A> {
20 
20 
21 error: defining use generics `[B, A]` differ from previous defining use
22   --> $DIR/multiple-def-uses-in-one-fn.rs:10:1

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn/multiple-def-uses-in-one-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/multiple-def-uses-in-one-fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: defining use generics `[B, A]` differ from previous defining use
  --> /checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs:10:45
   |
LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) {
   |
   |
note: previous defining use with different generics `[A, B]` found here
  --> /checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs:10:45
   |
LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) {


error[E0277]: the trait bound `&'static B: From<&A>` is not satisfied
  --> /checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs:7:16
   |
LL | type X<A, B> = impl Into<&'static A>;
   |                ^^^^^^^^^^^^^^^^^^^^^ the trait `From<&A>` is not implemented for `&'static B`
   |
   = note: required because of the requirements on the impl of `Into<&'static B>` for `&A`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) where &'static B: From<&A> {


error: defining use generics `[B, A]` differ from previous defining use
  --> /checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs:10:1
   |
LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) {
   |
   |
note: previous defining use with different generics `[A, B]` found here
  --> /checkout/src/test/ui/type-alias-impl-trait/multiple-def-uses-in-one-fn.rs:10:1
   |
LL | fn f<A, B: 'static>(a: &'static A, b: B) -> (X<A, B>, X<B, A>) {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 11412 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 134.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:37
