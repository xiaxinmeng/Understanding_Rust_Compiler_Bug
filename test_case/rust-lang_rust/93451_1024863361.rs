plain
.................................................................................................... 9700/12561
.................................................................................................... 9800/12561
.................................................................................................... 9900/12561
.................................................................................................... 10000/12561
................................................................................................i..i 10100/12561
.i..........F....F......F......F.........F.......F.................................................. 10200/12561
...............iiiiii..i.iiiiii.i................................................................... 10400/12561
.................................................................................................... 10500/12561
.................................................................................................... 10600/12561
.................................................................................................... 10700/12561
---
---- [ui] ui/rfc-2632-const-trait-impl/call-const-trait-method-fail.rs stdout ----
diff of stderr:

3    |
4 LL |     a.plus(b)
5    |       ^^^^^^^ the trait `~const Plus` is not implemented for `u32`
+    |
+ note: the trait `Plus` is implemented for `u32`, but that implementation is not `const`
+    |
+    |
+ LL |     a.plus(b)
6 
6 
7 error[E0015]: cannot call non-const fn `<u32 as Plus>::plus` in constant functions

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail/call-const-trait-method-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/call-const-trait-method-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `u32: ~const Plus` is not satisfied
   |
   |
LL |     a.plus(b)
   |       ^^^^^^^ the trait `~const Plus` is not implemented for `u32`
   |
note: the trait `Plus` is implemented for `u32`, but that implementation is not `const`
   |
   |
LL |     a.plus(b)


error[E0015]: cannot call non-const fn `<u32 as Plus>::plus` in constant functions
   |
   |
LL |     a.plus(b)
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
---
---- [ui] ui/rfc-2632-const-trait-impl/call-generic-method-fail.rs stdout ----
diff of stderr:

3    |
4 LL |     *t == *t
5    |     ^^^^^^^^ no implementation for `T == T`
+    |
+ note: the trait `PartialEq` is implemented for `T`, but that implementation is not `const`
+    |
+    |
+ LL |     *t == *t
6 
7 error[E0015]: cannot call non-const operator in constant functions
8   --> $DIR/call-generic-method-fail.rs:5:5

---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/call-generic-method-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/call-generic-method-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-generic-method-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-generic-method-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: can't compare `T` with `T` in const contexts
   |
   |
LL |     *t == *t
   |     ^^^^^^^^ no implementation for `T == T`
   |
note: the trait `PartialEq` is implemented for `T`, but that implementation is not `const`
   |
   |
LL |     *t == *t

error[E0015]: cannot call non-const operator in constant functions
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/call-generic-method-fail.rs:5:5
   |
   |
LL |     *t == *t
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
   |
LL | pub const fn equals_self<T: PartialEq + ~const std::cmp::PartialEq>(t: &T) -> bool {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
---
---- [ui] ui/rfc-2632-const-trait-impl/const-default-method-bodies.rs stdout ----
diff of stderr:

3    |
4 LL |     NonConstImpl.a();
5    |                  ^^^ the trait `~const ConstDefaultFn` is not implemented for `NonConstImpl`
+    |
+ note: the trait `ConstDefaultFn` is implemented for `NonConstImpl`, but that implementation is not `const`
+    |
+    |
+ LL |     NonConstImpl.a();
6 
6 
7 error[E0015]: cannot call non-const fn `<NonConstImpl as ConstDefaultFn>::a` in constant functions


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-default-method-bodies/const-default-method-bodies.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-default-method-bodies/const-default-method-bodies.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-default-method-bodies.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-default-method-bodies.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-default-method-bodies" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-default-method-bodies/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `NonConstImpl: ~const ConstDefaultFn` is not satisfied
   |
   |
LL |     NonConstImpl.a();
   |                  ^^^ the trait `~const ConstDefaultFn` is not implemented for `NonConstImpl`
   |
note: the trait `ConstDefaultFn` is implemented for `NonConstImpl`, but that implementation is not `const`
   |
   |
LL |     NonConstImpl.a();


error[E0015]: cannot call non-const fn `<NonConstImpl as ConstDefaultFn>::a` in constant functions
   |
   |
LL |     NonConstImpl.a();
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
---
---- [ui] ui/rfc-2632-const-trait-impl/default-method-body-is-const-same-trait-ck.rs stdout ----
diff of stderr:

3    |
4 LL |         ().a()
5    |            ^^^ the trait `~const Tr` is not implemented for `()`
+    |
+ note: the trait `Tr` is implemented for `()`, but that implementation is not `const`
+    |
+    |
+ LL |         ().a()
6 
6 
7 error[E0015]: cannot call non-const fn `<() as Tr>::a` in constant functions


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/default-method-body-is-const-same-trait-ck/default-method-body-is-const-same-trait-ck.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/default-method-body-is-const-same-trait-ck.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/default-method-body-is-const-same-trait-ck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/default-method-body-is-const-same-trait-ck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/default-method-body-is-const-same-trait-ck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `(): ~const Tr` is not satisfied
   |
LL |         ().a()
LL |         ().a()
   |            ^^^ the trait `~const Tr` is not implemented for `()`
   |
note: the trait `Tr` is implemented for `()`, but that implementation is not `const`
   |
LL |         ().a()
   |            ^^^


error[E0015]: cannot call non-const fn `<() as Tr>::a` in constant functions
   |
LL |         ().a()
   |            ^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
---
---- [ui] ui/rfc-2632-const-trait-impl/cross-crate.rs#gated stdout ----
diff of stderr:

3    |
4 LL |     NonConst.func();
5    |              ^^^^^^ the trait `~const cross_crate::MyTrait` is not implemented for `cross_crate::NonConst`
+    |
+ note: the trait `cross_crate::MyTrait` is implemented for `cross_crate::NonConst`, but that implementation is not `const`
+    |
+ LL |     NonConst.func();
+    |              ^^^^^^
6 
6 
7 error[E0015]: cannot call non-const fn `<cross_crate::NonConst as cross_crate::MyTrait>::func` in constant functions


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/cross-crate.gated/cross-crate.gated.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/cross-crate.rs`


error in revision `gated`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "gated" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/cross-crate.gated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/cross-crate.gated/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `cross_crate::NonConst: ~const cross_crate::MyTrait` is not satisfied
   |
   |
LL |     NonConst.func(); //~ ERROR: cannot call non-const fn
   |              ^^^^^^ the trait `~const cross_crate::MyTrait` is not implemented for `cross_crate::NonConst`
   |
note: the trait `cross_crate::MyTrait` is implemented for `cross_crate::NonConst`, but that implementation is not `const`
   |
   |
LL |     NonConst.func(); //~ ERROR: cannot call non-const fn


error[E0015]: cannot call non-const fn `<cross_crate::NonConst as cross_crate::MyTrait>::func` in constant functions
   |
   |
LL |     NonConst.func(); //~ ERROR: cannot call non-const fn
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
---
---- [ui] ui/rfc-2632-const-trait-impl/issue-88155.rs stdout ----
diff of stderr:

3    |
4 LL |     T::assoc()
5    |     ^^^^^^^^^^ the trait `~const A` is not implemented for `T`
+    |
+ note: the trait `A` is implemented for `T`, but that implementation is not `const`
+    |
+    |
+ LL |     T::assoc()
6 
6 
7 error[E0015]: cannot call non-const fn `<T as A>::assoc` in constant functions


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-88155/issue-88155.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-88155/issue-88155.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/issue-88155.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/issue-88155.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-88155" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/issue-88155/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `T: ~const A` is not satisfied
   |
LL |     T::assoc()
LL |     T::assoc()
   |     ^^^^^^^^^^ the trait `~const A` is not implemented for `T`
   |
note: the trait `A` is implemented for `T`, but that implementation is not `const`
   |
LL |     T::assoc()
   |     ^^^^^^^^^^


error[E0015]: cannot call non-const fn `<T as A>::assoc` in constant functions
   |
LL |     T::assoc()
   |     ^^^^^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
---
test result: FAILED. 12434 passed; 6 failed; 121 ignored; 0 measured; 0 filtered out; finished in 102.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:31
