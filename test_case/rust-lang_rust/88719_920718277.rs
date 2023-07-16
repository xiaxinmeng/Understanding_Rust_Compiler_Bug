plain

---- [ui] ui/rfc-2632-const-trait-impl/const-drop-fail.rs#precise stdout ----
diff of stderr:

9 error[E0277]: the trait bound `NonTrivialDrop: Drop` is not satisfied
11    |
11    |
+ LL |         const _: () = check($exp);
+    |                       ----- required by a bound introduced by this call
+ ...
12 LL |     NonTrivialDrop,
13    |     ^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `NonTrivialDrop`


21 error[E0277]: the trait bound `ConstImplWithDropGlue: Drop` is not satisfied
23    |
23    |
+ LL |         const _: () = check($exp);
+    |                       ----- required by a bound introduced by this call
+ ...
24 LL |     ConstImplWithDropGlue(NonTrivialDrop),
25    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `ConstImplWithDropGlue`


45 error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
47    |
47    |
+ LL |         const _: () = check($exp);
+    |                       ----- required by a bound introduced by this call
+ ...
48 LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
49    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise/const-drop-fail.precise.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-drop-fail.rs`


error in revision `precise`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "precise" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.precise/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `~const` is not allowed here
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |
   |
   = note: only allowed on bounds on traits' associated types and functions, const fns, const impls and its associated functions

error[E0277]: the trait bound `NonTrivialDrop: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     NonTrivialDrop,
   |     ^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `NonTrivialDrop`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `ConstImplWithDropGlue: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `ConstImplWithDropGlue`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);


error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by a bound in `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |                                   ^^^^^^^^ required by this bound in `ConstDropImplWithBounds`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/rfc-2632-const-trait-impl/const-drop-fail.rs#stock stdout ----
diff of stderr:

9 error[E0277]: the trait bound `NonTrivialDrop: Drop` is not satisfied
11    |
11    |
+ LL |         const _: () = check($exp);
+    |                       ----- required by a bound introduced by this call
+ ...
12 LL |     NonTrivialDrop,
13    |     ^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `NonTrivialDrop`


21 error[E0277]: the trait bound `ConstImplWithDropGlue: Drop` is not satisfied
23    |
23    |
+ LL |         const _: () = check($exp);
+    |                       ----- required by a bound introduced by this call
+ ...
24 LL |     ConstImplWithDropGlue(NonTrivialDrop),
25    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `ConstImplWithDropGlue`


45 error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
47    |
47    |
+ LL |         const _: () = check($exp);
+    |                       ----- required by a bound introduced by this call
+ ...
48 LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
49    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock/const-drop-fail.stock.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-drop-fail.rs`


error in revision `stock`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "stock" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-drop-fail.stock/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `~const` is not allowed here
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |
   |
   = note: only allowed on bounds on traits' associated types and functions, const fns, const impls and its associated functions

error[E0277]: the trait bound `NonTrivialDrop: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     NonTrivialDrop,
   |     ^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `NonTrivialDrop`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `ConstImplWithDropGlue: Drop` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     ConstImplWithDropGlue(NonTrivialDrop),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Drop` is not implemented for `ConstImplWithDropGlue`
note: required by a bound in `check`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-drop-fail.rs:36:19
   |
   |
LL | const fn check<T: ~const Drop>(_: T) {}
   |                   ^^^^^^^^^^^ required by this bound in `check`

error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);


error[E0277]: the trait bound `NonTrivialDrop: A` is not satisfied
   |
   |
LL |         const _: () = check($exp);
   |                       ----- required by a bound introduced by this call
...
LL |     ConstDropImplWithBounds::<NonTrivialDrop>(PhantomData),
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `A` is not implemented for `NonTrivialDrop`
   |
note: required by a bound in `ConstDropImplWithBounds`
   |
   |
LL | struct ConstDropImplWithBounds<T: ~const A>(PhantomData<T>);
   |                                   ^^^^^^^^ required by this bound in `ConstDropImplWithBounds`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 12036 passed; 2 failed; 102 ignored; 0 measured; 0 filtered out; finished in 129.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:21
