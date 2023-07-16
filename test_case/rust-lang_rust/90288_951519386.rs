plain
...............................i.ii................................................................. 12300/12344
............................................
failures:

---- [ui] ui/suggestions/suggest-tryinto-edition-change.rs stdout ----

1 error[E0433]: failed to resolve: use of undeclared type `TryFrom`
1 error[E0433]: failed to resolve: use of undeclared type `TryFrom`
-   --> $DIR/suggest-tryinto-edition-change.rs:14:30
+   --> $DIR/suggest-tryinto-edition-change.rs:16:30
3    |
4 LL |     let _i: Result<i16, _> = TryFrom::try_from(0_i32);

11    |
12 
13 error[E0433]: failed to resolve: use of undeclared type `TryInto`
13 error[E0433]: failed to resolve: use of undeclared type `TryInto`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/suggest-tryinto-edition-change.rs:19:30
+   --> $DIR/suggest-tryinto-edition-change.rs:21:30
15    |
16 LL |     let _i: Result<i16, _> = TryInto::try_into(0_i32);

23    |
24 
25 error[E0433]: failed to resolve: use of undeclared type `FromIterator`
25 error[E0433]: failed to resolve: use of undeclared type `FromIterator`
-   --> $DIR/suggest-tryinto-edition-change.rs:24:18
+   --> $DIR/suggest-tryinto-edition-change.rs:26:18
27    |
28 LL |     let _i: () = FromIterator::from_iter(core::iter::empty());

44    |
45 
45 
46 error[E0599]: no method named `try_into` found for type `i32` in the current scope
-   --> $DIR/suggest-tryinto-edition-change.rs:9:36
+   --> $DIR/suggest-tryinto-edition-change.rs:11:36
48    |
49 LL |     let _i: Result<i16, _> = 0_i32.try_into();
50    |                                    ^^^^^^^^ method not found in `i32`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-tryinto-edition-change/suggest-tryinto-edition-change.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-tryinto-edition-change.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-tryinto-edition-change.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-tryinto-edition-change" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-tryinto-edition-change/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0433]: failed to resolve: use of undeclared type `TryFrom`
  --> /checkout/src/test/ui/suggestions/suggest-tryinto-edition-change.rs:16:30
   |
LL |     let _i: Result<i16, _> = TryFrom::try_from(0_i32);
   |
   |
   = note: 'core::convert::TryFrom' is included in the prelude starting in Edition 2021
   |
LL | use core::convert::TryFrom;
   |


error[E0433]: failed to resolve: use of undeclared type `TryInto`
  --> /checkout/src/test/ui/suggestions/suggest-tryinto-edition-change.rs:21:30
   |
LL |     let _i: Result<i16, _> = TryInto::try_into(0_i32);
   |
   |
   = note: 'core::convert::TryInto' is included in the prelude starting in Edition 2021
   |
LL | use core::convert::TryInto;
   |


error[E0433]: failed to resolve: use of undeclared type `FromIterator`
  --> /checkout/src/test/ui/suggestions/suggest-tryinto-edition-change.rs:26:18
   |
LL |     let _i: () = FromIterator::from_iter(core::iter::empty());
   |
  ::: /checkout/library/core/src/iter/traits/collect.rs:204:1
   |
LL | pub trait IntoIterator {
LL | pub trait IntoIterator {
   | ---------------------- similarly named trait `IntoIterator` defined here
   |
   = note: 'core::iter::FromIterator' is included in the prelude starting in Edition 2021
help: a trait with a similar name exists
   |
LL |     let _i: () = IntoIterator::from_iter(core::iter::empty());
help: consider importing this trait
   |
LL | use core::iter::FromIterator;
   |
   |

error[E0599]: no method named `try_into` found for type `i32` in the current scope
  --> /checkout/src/test/ui/suggestions/suggest-tryinto-edition-change.rs:11:36
   |
LL |     let _i: Result<i16, _> = 0_i32.try_into();
   |                                    ^^^^^^^^ method not found in `i32`
  ::: /checkout/library/core/src/convert/mod.rs:399:8
   |
LL |     fn try_into(self) -> Result<T, Self::Error>;
   |        -------- the method is available for `i32` here
   |        -------- the method is available for `i32` here
   |
   = help: items from traits can only be used if the trait is in scope
   = note: 'core::convert::TryInto' is included in the prelude starting in Edition 2021
   |
LL | use core::convert::TryInto;
   |

---
test result: FAILED. 12233 passed; 1 failed; 110 ignored; 0 measured; 0 filtered out; finished in 133.15s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:54
