plain

---- [ui (nll)] ui/associated-types/associated-types-eq-hr.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `for<'x> <UintStruct as TheTrait<&'x isize>>::A == &'x isize`
3    |
- LL | fn foo<T>()
-    |    --- required by a bound in this
- LL | where
- LL | where
- LL |     T: for<'x> TheTrait<&'x isize, A = &'x isize>,
-    |                                    ------------- required by this bound in `foo`
- ...
10 LL |     foo::<UintStruct>();
11    |     ^^^^^^^^^^^^^^^^^ expected `isize`, found `usize`

13    = note: expected reference `&isize`
14               found reference `&usize`
+ note: required by a bound in `foo`
+ note: required by a bound in `foo`
+   --> $DIR/associated-types-eq-hr.rs:45:36
+    |
+ LL | fn foo<T>()
+    |    --- required by a bound in this
+ LL | where
+ LL |     T: for<'x> TheTrait<&'x isize, A = &'x isize>,
+    |                                    ^^^^^^^^^^^^^ required by this bound in `foo`
15 
16 error[E0271]: type mismatch resolving `for<'x> <IntStruct as TheTrait<&'x isize>>::A == &'x usize`
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

18    |
18    |
- LL | fn bar<T>()
-    |    --- required by a bound in this
- LL | where
- LL |     T: for<'x> TheTrait<&'x isize, A = &'x usize>,
-    |                                    ------------- required by this bound in `bar`
- ...
25 LL |     bar::<IntStruct>();
26    |     ^^^^^^^^^^^^^^^^ expected `usize`, found `isize`

28    = note: expected reference `&usize`
29               found reference `&isize`
29               found reference `&isize`
+ note: required by a bound in `bar`
+    |
+    |
+ LL | fn bar<T>()
+    |    --- required by a bound in this
+ LL | where
+ LL |     T: for<'x> TheTrait<&'x isize, A = &'x usize>,
+    |                                    ^^^^^^^^^^^^^ required by this bound in `bar`
31 error: aborting due to 2 previous errors
32 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-hr.nll/associated-types-eq-hr.nll.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-eq-hr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-eq-hr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-hr.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-hr.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `for<'x> <UintStruct as TheTrait<&'x isize>>::A == &'x isize`
   |
   |
LL |     foo::<UintStruct>(); //~ ERROR type mismatch
   |     ^^^^^^^^^^^^^^^^^ expected `isize`, found `usize`
   = note: expected reference `&isize`
              found reference `&usize`
note: required by a bound in `foo`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-hr.rs:45:36
  --> /checkout/src/test/ui/associated-types/associated-types-eq-hr.rs:45:36
   |
LL | fn foo<T>()
   |    --- required by a bound in this
LL | where
LL |     T: for<'x> TheTrait<&'x isize, A = &'x isize>,
   |                                    ^^^^^^^^^^^^^ required by this bound in `foo`

error[E0271]: type mismatch resolving `for<'x> <IntStruct as TheTrait<&'x isize>>::A == &'x usize`
   |
   |
LL |     bar::<IntStruct>(); //~ ERROR type mismatch
   |     ^^^^^^^^^^^^^^^^ expected `usize`, found `isize`
   = note: expected reference `&usize`
              found reference `&isize`
              found reference `&isize`
note: required by a bound in `bar`
   |
   |
LL | fn bar<T>()
   |    --- required by a bound in this
LL | where
LL |     T: for<'x> TheTrait<&'x isize, A = &'x usize>,
   |                                    ^^^^^^^^^^^^^ required by this bound in `bar`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui (nll)] ui/kindck/kindck-send-object1.rs stdout ----
diff of stderr:

1 error[E0277]: `(dyn Dummy + 'a)` cannot be shared between threads safely
2   --> $DIR/kindck-send-object1.rs:10:5
3    |
- LL | fn assert_send<T:Send+'static>() { }
-    |                  ---- required by this bound in `assert_send`
- ...
7 LL |     assert_send::<&'a dyn Dummy>();
8    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely


10    = help: the trait `Sync` is not implemented for `(dyn Dummy + 'a)`
11    = note: required because of the requirements on the impl of `Send` for `&'a (dyn Dummy + 'a)`
+ note: required by a bound in `assert_send`
+   --> $DIR/kindck-send-object1.rs:5:18
+    |
+ LL | fn assert_send<T:Send+'static>() { }
+    |                  ^^^^ required by this bound in `assert_send`
12 
13 error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
14   --> $DIR/kindck-send-object1.rs:29:5
15    |
15    |
- LL | fn assert_send<T:Send+'static>() { }
-    |                  ---- required by this bound in `assert_send`
- ...
19 LL |     assert_send::<Box<dyn Dummy + 'a>>();
20    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely


22    = help: the trait `Send` is not implemented for `(dyn Dummy + 'a)`
23    = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Dummy + 'a)>`
24    = note: required because it appears within the type `Box<(dyn Dummy + 'a)>`
+ note: required by a bound in `assert_send`
+   --> $DIR/kindck-send-object1.rs:5:18
+    |
+ LL | fn assert_send<T:Send+'static>() { }
+    |                  ^^^^ required by this bound in `assert_send`
26 error: aborting due to 2 previous errors
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll/kindck-send-object1.nll.stderr
To only update this specific test, also pass `--test-args kindck/kindck-send-object1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-send-object1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `(dyn Dummy + 'a)` cannot be shared between threads safely
  --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:10:5
   |
LL |     assert_send::<&'a dyn Dummy>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `(dyn Dummy + 'a)`
   = note: required because of the requirements on the impl of `Send` for `&'a (dyn Dummy + 'a)`
note: required by a bound in `assert_send`
  --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:5:18
   |
LL | fn assert_send<T:Send+'static>() { }
   |                  ^^^^ required by this bound in `assert_send`

error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
  --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:29:5
   |
LL |     assert_send::<Box<dyn Dummy + 'a>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `(dyn Dummy + 'a)`
   = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Dummy + 'a)>`
   = note: required because it appears within the type `Box<(dyn Dummy + 'a)>`
note: required by a bound in `assert_send`
  --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:5:18
   |
LL | fn assert_send<T:Send+'static>() { }
   |                  ^^^^ required by this bound in `assert_send`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 12002 passed; 2 failed; 129 ignored; 0 measured; 0 filtered out; finished in 102.89s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:21:10
