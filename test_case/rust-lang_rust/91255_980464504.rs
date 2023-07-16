plain

---- [ui] ui/associated-types/issue-59324.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `Bug: Foo` is not satisfied
-   --> $DIR/issue-59342.rs:11:1
3    |
3    |
4 LL | / pub trait ThriftService<Bug: NotFoo>:
5 LL | |
16    |                                     +++++
17 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
18 error[E0277]: the trait bound `Bug: Foo` is not satisfied
-   --> $DIR/issue-59342.rs:11:1
20    |
20    |
21 LL | / pub trait ThriftService<Bug: NotFoo>:
22 LL | |
33    |                                     +++++
34 
34 
35 error[E0277]: the trait bound `Bug: Foo` is not satisfied
-   --> $DIR/issue-59342.rs:16:5
37    |
38 LL | /     fn get_service(
39 LL | |


48    |                                     +++++
49 
50 error[E0277]: the trait bound `Bug: Foo` is not satisfied
-   --> $DIR/issue-59342.rs:16:8
52    |
53 LL |     fn get_service(
54    |        ^^^^^^^^^^^ the trait `Foo` is not implemented for `Bug`


59    |                                     +++++
60 
61 error[E0277]: the trait bound `(): Foo` is not satisfied
-   --> $DIR/issue-59342.rs:23:29
63    |
63    |
64 LL | fn with_factory<H>(factory: dyn ThriftService<()>) {}
65    |                             ^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `()`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-59324/issue-59324.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-59324.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-59324.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-59324" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-59324/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Bug: Foo` is not satisfied
   |
   |
LL | / pub trait ThriftService<Bug: NotFoo>:
LL | | //~^ ERROR the trait bound `Bug: Foo` is not satisfied
LL | | //~| ERROR the trait bound `Bug: Foo` is not satisfied
LL | |     Service<AssocType = <Bug as Foo>::OnlyFoo>
LL | |     ) -> Self::AssocType;
LL | | }
LL | | }
   | |_^ the trait `Foo` is not implemented for `Bug`
help: consider further restricting this bound
   |
   |
LL | pub trait ThriftService<Bug: NotFoo + Foo>:


error[E0277]: the trait bound `Bug: Foo` is not satisfied
   |
   |
LL | / pub trait ThriftService<Bug: NotFoo>:
LL | | //~^ ERROR the trait bound `Bug: Foo` is not satisfied
LL | | //~| ERROR the trait bound `Bug: Foo` is not satisfied
LL | |     Service<AssocType = <Bug as Foo>::OnlyFoo>
LL | |     ) -> Self::AssocType;
LL | | }
LL | | }
   | |_^ the trait `Foo` is not implemented for `Bug`
help: consider further restricting this bound
   |
   |
LL | pub trait ThriftService<Bug: NotFoo + Foo>:


error[E0277]: the trait bound `Bug: Foo` is not satisfied
   |
LL | /     fn get_service(
LL | /     fn get_service(
LL | |     //~^ ERROR the trait bound `Bug: Foo` is not satisfied
LL | |     //~| ERROR the trait bound `Bug: Foo` is not satisfied
LL | |         &self,
LL | |     ) -> Self::AssocType;
   | |_________________________^ the trait `Foo` is not implemented for `Bug`
help: consider further restricting this bound
   |
   |
LL | pub trait ThriftService<Bug: NotFoo + Foo>:


error[E0277]: the trait bound `Bug: Foo` is not satisfied
   |
LL |     fn get_service(
   |        ^^^^^^^^^^^ the trait `Foo` is not implemented for `Bug`
   |
   |
help: consider further restricting this bound
   |
LL | pub trait ThriftService<Bug: NotFoo + Foo>:


error[E0277]: the trait bound `(): Foo` is not satisfied
   |
   |
LL | fn with_factory<H>(factory: dyn ThriftService<()>) {}
   |                             ^^^^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `()`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
diff of stderr:

16    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
17    = note: ...which requires computing layout of `Foo`...
18    = note: ...which requires computing layout of `[u8; _]`...
-    = note: ...which requires trying to normalize `[u8; _]`...
+    = note: ...which requires normalizing `[u8; _]`...
20    = note: ...which again requires simplifying constant for the type system `Foo::bytes::{constant#0}`, completing the cycle
21 note: cycle used when checking that `Foo` is well-formed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `Foo::bytes::{constant#0}`
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |
   |
note: ...which requires simplifying constant for the type system `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; std::mem::size_of::<Foo>()]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires computing layout of `Foo`...
   = note: ...which requires computing layout of `[u8; _]`...
   = note: ...which requires normalizing `[u8; _]`...
   = note: ...which again requires simplifying constant for the type system `Foo::bytes::{constant#0}`, completing the cycle
note: cycle used when checking that `Foo` is well-formed
   |
LL | struct Foo {
   | ^^^^^^^^^^

---
---- [ui] ui/consts/issue-44415.rs stdout ----
diff of stderr:

16    |                 ^^^^^^
17    = note: ...which requires computing layout of `Foo`...
18    = note: ...which requires computing layout of `[u8; _]`...
-    = note: ...which requires trying to normalize `[u8; _]`...
+    = note: ...which requires normalizing `[u8; _]`...
20    = note: ...which again requires simplifying constant for the type system `Foo::bytes::{constant#0}`, completing the cycle
21 note: cycle used when checking that `Foo` is well-formed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415/issue-44415.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415/issue-44415.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-44415.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-44415.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-44415/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when simplifying constant for the type system `Foo::bytes::{constant#0}`
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |
   |
note: ...which requires simplifying constant for the type system `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |                 ^^^^^^
note: ...which requires const-evaluating + checking `Foo::bytes::{constant#0}`...
   |
   |
LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
   |                 ^^^^^^
   = note: ...which requires computing layout of `Foo`...
   = note: ...which requires computing layout of `[u8; _]`...
   = note: ...which requires normalizing `[u8; _]`...
   = note: ...which again requires simplifying constant for the type system `Foo::bytes::{constant#0}`, completing the cycle
note: cycle used when checking that `Foo` is well-formed
   |
LL | struct Foo {
   | ^^^^^^^^^^

---
test result: FAILED. 12310 passed; 3 failed; 111 ignored; 0 measured; 0 filtered out; finished in 142.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:31
