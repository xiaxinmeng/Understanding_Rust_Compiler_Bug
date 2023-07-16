plain
diff of stderr:

2   --> $DIR/issue-87429-associated-type-default.rs:14:5
3    |
4 LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>> = Foo;
-    |     |                |
-    |     |                |
-    |     |                required by this bound in `Family2::Member`
-    |     no implementation for `Foo == Foo`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`
9    |
10    = help: the trait `PartialEq` is not implemented for `Foo`
+ note: required by a bound in `Family2::Member`
+    |
+    |
+ LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>> = Foo;
+    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Family2::Member`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-associated-type-default/issue-87429-associated-type-default.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-87429-associated-type-default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87429-associated-type-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-associated-type-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-associated-type-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: can't compare `Foo` with `Foo`
  --> /checkout/src/test/ui/generic-associated-types/issue-87429-associated-type-default.rs:14:5
   |
LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>> = Foo;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`
   |
   = help: the trait `PartialEq` is not implemented for `Foo`
note: required by a bound in `Family2::Member`
   |
   |
LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>> = Foo;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Family2::Member`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/generic-associated-types/issue-87429-specialization.rs stdout ----

11 error[E0277]: can't compare `Foo` with `Foo`
12   --> $DIR/issue-87429-specialization.rs:21:5
13    |
13    |
- LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>>;
-    |                      ----------------------------------- required by this bound in `Family::Member`
- ...
17 LL |     default type Member<'a> = Foo;
18    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`


20    = help: the trait `PartialEq` is not implemented for `Foo`
+ note: required by a bound in `Family::Member`
+   --> $DIR/issue-87429-specialization.rs:8:22
+    |
+ LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>>;
+    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Family::Member`
22 error: aborting due to previous error; 1 warning emitted
23 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization/issue-87429-specialization.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-87429-specialization.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87429-specialization/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs:3:12
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0277]: can't compare `Foo` with `Foo`
  --> /checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs:21:5
   |
LL |     default type Member<'a> = Foo;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `Foo == Foo`
   |
   = help: the trait `PartialEq` is not implemented for `Foo`
note: required by a bound in `Family::Member`
  --> /checkout/src/test/ui/generic-associated-types/issue-87429-specialization.rs:8:22
   |
LL |     type Member<'a>: for<'b> PartialEq<Self::Member<'b>>;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Family::Member`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 12073 passed; 2 failed; 102 ignored; 0 measured; 0 filtered out; finished in 125.70s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:23
