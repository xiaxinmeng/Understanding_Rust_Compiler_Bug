plain
diff of stderr:

11    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `u32` is not a future
12    |
13    = help: the trait `Future` is not implemented for `u32`
+    = note: u32 must be a future or must implement `IntoFuture` to be awaited
15 error: aborting due to 2 previous errors
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues-71798/issues-71798.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args issues-71798.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues-71798.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues-71798" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues-71798/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `u` in this scope
  --> /checkout/src/test/ui/issues-71798.rs:6:24
   |
LL |     let _ = test_ref & u; //~ ERROR cannot find value `u` in this scope
   |                        ^ not found in this scope
error[E0277]: `u32` is not a future
  --> /checkout/src/test/ui/issues-71798.rs:1:25
   |
   |
LL | fn test_ref(x: &u32) -> impl std::future::Future<Output = u32> + '_ {
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `u32` is not a future
   |
   = help: the trait `Future` is not implemented for `u32`
   = note: u32 must be a future or must implement `IntoFuture` to be awaited
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0425.
For more information about an error, try `rustc --explain E0277`.
---
diff of stderr:

10    |     required by a bound introduced by this call
11    |
12    = help: the trait `Future` is not implemented for `fn() -> impl Future<Output = ()> {foo}`
+    = note: fn() -> impl Future<Output = ()> {foo} must be a future or must implement `IntoFuture` to be awaited
13 note: required by a bound in `bar`
15    |

31    |     required by a bound introduced by this call
32    |
32    |
33    = help: the trait `Future` is not implemented for `[closure@$DIR/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]`
+    = note: [closure@$DIR/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36] must be a future or must implement `IntoFuture` to be awaited
34 note: required by a bound in `bar`
36    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `fn() -> impl Future<Output = ()> {foo}` is not a future
   |
   |
LL | async fn foo() {}
...
...
LL |     bar(foo); //~ERROR E0277
   |     --- ^^^ `fn() -> impl Future<Output = ()> {foo}` is not a future
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Future` is not implemented for `fn() -> impl Future<Output = ()> {foo}`
   = note: fn() -> impl Future<Output = ()> {foo} must be a future or must implement `IntoFuture` to be awaited
note: required by a bound in `bar`
   |
   |
LL | fn bar(f: impl Future<Output=()>) {}
   |                ^^^^^^^^^^^^^^^^^ required by this bound in `bar`
   |
   |
LL |     bar(foo()); //~ERROR E0277


error[E0277]: `[closure@/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]` is not a future
   |
   |
LL |     let async_closure = async || ();
   |                         -------- consider calling this closure
LL |     bar(async_closure); //~ERROR E0277
   |     --- ^^^^^^^^^^^^^ `[closure@/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]` is not a future
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Future` is not implemented for `[closure@/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36]`
   = note: [closure@/checkout/src/test/ui/suggestions/async-fn-ctor-passed-as-arg-where-it-should-have-been-called.rs:11:25: 11:36] must be a future or must implement `IntoFuture` to be awaited
note: required by a bound in `bar`
   |
   |
LL | fn bar(f: impl Future<Output=()>) {}
   |                ^^^^^^^^^^^^^^^^^ required by this bound in `bar`
help: use parentheses to call the closure
   |
LL |     bar(async_closure()); //~ERROR E0277

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 12318 passed; 2 failed; 115 ignored; 0 measured; 0 filtered out; finished in 145.89s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:21
