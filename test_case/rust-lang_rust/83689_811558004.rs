plain

---- [ui (nll)] ui/kindck/kindck-impl-type-params.rs stdout ----
diff of stderr:

4 LL |     let a = &t as &dyn Gettable<T>;
5    |             ^^ `T` cannot be sent between threads safely
6    |
-    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+ note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+   --> $DIR/kindck-impl-type-params.rs:14:32
+    |
+ LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
+    |                                ^^^^^^^^^^^     ^^^^
8    = note: required for the cast to the object type `dyn Gettable<T>`
9 help: consider restricting type parameter `T`


17 LL |     let a = &t as &dyn Gettable<T>;
18    |             ^^ the trait `Copy` is not implemented for `T`
19    |
-    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+ note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+   --> $DIR/kindck-impl-type-params.rs:14:32
+    |
+ LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
+    |                                ^^^^^^^^^^^     ^^^^
21    = note: required for the cast to the object type `dyn Gettable<T>`
22 help: consider restricting type parameter `T`


30 LL |     let a: &dyn Gettable<T> = &t;
31    |                               ^^ `T` cannot be sent between threads safely
32    |
-    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+ note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+   --> $DIR/kindck-impl-type-params.rs:14:32
+    |
+ LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
+    |                                ^^^^^^^^^^^     ^^^^
34    = note: required for the cast to the object type `dyn Gettable<T>`
35 help: consider restricting type parameter `T`


43 LL |     let a: &dyn Gettable<T> = &t;
44    |                               ^^ the trait `Copy` is not implemented for `T`
45    |
-    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+ note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
+   --> $DIR/kindck-impl-type-params.rs:14:32
+    |
+ LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
+    |                                ^^^^^^^^^^^     ^^^^
47    = note: required for the cast to the object type `dyn Gettable<T>`
48 help: consider restricting type parameter `T`


56 LL |     let a = t as Box<dyn Gettable<String>>;
57    |             ^ the trait `Copy` is not implemented for `String`
58    |
-    = note: required because of the requirements on the impl of `Gettable<String>` for `S<String>`
+ note: required because of the requirements on the impl of `Gettable<String>` for `S<String>`
+   --> $DIR/kindck-impl-type-params.rs:14:32
+    |
+ LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |                                ^^^^^^^^^^^     ^^^^
60    = note: required for the cast to the object type `dyn Gettable<String>`
61 
62 error[E0277]: the trait bound `Foo: Copy` is not satisfied

65 LL |     let a: Box<dyn Gettable<Foo>> = t;
66    |                                     ^ the trait `Copy` is not implemented for `Foo`
67    |
-    = note: required because of the requirements on the impl of `Gettable<Foo>` for `S<Foo>`
+ note: required because of the requirements on the impl of `Gettable<Foo>` for `S<Foo>`
+   --> $DIR/kindck-impl-type-params.rs:14:32
+    |
+ LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
+    |                                ^^^^^^^^^^^     ^^^^
69    = note: required for the cast to the object type `dyn Gettable<Foo>`
71 error: aborting due to 6 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/kindck-impl-type-params.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args kindck/kindck-impl-type-params.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-impl-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `T` cannot be sent between threads safely
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
   |
LL |     let a = &t as &dyn Gettable<T>;
   |             ^^ `T` cannot be sent between threads safely
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
LL | fn f<T: Send>(val: T) {


error[E0277]: the trait bound `T: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
   |
LL |     let a = &t as &dyn Gettable<T>;
   |             ^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
LL | fn f<T: Copy>(val: T) {


error[E0277]: `T` cannot be sent between threads safely
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
   |
LL |     let a: &dyn Gettable<T> = &t;
   |                               ^^ `T` cannot be sent between threads safely
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
LL | fn g<T: Send>(val: T) {


error[E0277]: the trait bound `T: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
   |
LL |     let a: &dyn Gettable<T> = &t;
   |                               ^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
LL | fn g<T: Copy>(val: T) {

error[E0277]: the trait bound `String: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:38:13
   |
   |
LL |     let a = t as Box<dyn Gettable<String>>;
   |             ^ the trait `Copy` is not implemented for `String`
   |
note: required because of the requirements on the impl of `Gettable<String>` for `S<String>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<String>`

error[E0277]: the trait bound `Foo: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:46:37
   |
LL |     let a: Box<dyn Gettable<Foo>> = t;
   |                                     ^ the trait `Copy` is not implemented for `Foo`
   |
note: required because of the requirements on the impl of `Gettable<Foo>` for `S<Foo>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<Foo>`
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui (nll)] ui/rfc1623.rs stdout ----
diff of stderr:

11    |
12    = help: within `&SomeStruct`, the trait `Sync` is not implemented for `dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>`
13    = note: required because it appears within the type `&dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>`
-    = note: required because it appears within the type `SomeStruct`
+ note: required because it appears within the type `SomeStruct`
+   --> $DIR/rfc1623.rs:11:8
+    |
+ LL | struct SomeStruct<'x, 'y, 'z: 'x> {
+    |        ^^^^^^^^^^
15    = note: required because it appears within the type `&SomeStruct`
16    = note: shared static variables must have a type that implements `Sync`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623.nll/rfc1623.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc1623.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1623.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>` cannot be shared between threads safely
   |
   |
LL | / static SOME_STRUCT: &SomeStruct = &SomeStruct {
LL | |     foo: &Foo { bools: &[false, true] },
LL | |     bar: &Bar { bools: &[true, true] },
LL | |     f: &id,
LL | |     //~^ ERROR implementation of `FnOnce` is not general enough
LL | | };
   | |__^ `dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>` cannot be shared between threads safely
   |
   = help: within `&SomeStruct`, the trait `Sync` is not implemented for `dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>`
   = note: required because it appears within the type `&dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>`
note: required because it appears within the type `SomeStruct`
   |
   |
LL | struct SomeStruct<'x, 'y, 'z: 'x> {
   |        ^^^^^^^^^^
   = note: required because it appears within the type `&SomeStruct`
   = note: shared static variables must have a type that implements `Sync`
error: higher-ranked subtype error
  --> /checkout/src/test/ui/rfc1623.rs:21:35
   |
   |
LL |   static SOME_STRUCT: &SomeStruct = &SomeStruct {
   |  ___________________________________^
LL | |     foo: &Foo { bools: &[false, true] },
LL | |     bar: &Bar { bools: &[true, true] },
LL | |     f: &id,
LL | |     //~^ ERROR implementation of `FnOnce` is not general enough
LL | | };

error: higher-ranked subtype error
  --> /checkout/src/test/ui/rfc1623.rs:21:35
   |
   |
LL |   static SOME_STRUCT: &SomeStruct = &SomeStruct {
   |  ___________________________________^
LL | |     foo: &Foo { bools: &[false, true] },
LL | |     bar: &Bar { bools: &[true, true] },
LL | |     f: &id,
LL | |     //~^ ERROR implementation of `FnOnce` is not general enough
LL | | };

error: higher-ranked subtype error
  --> /checkout/src/test/ui/rfc1623.rs:21:35
   |
   |
LL |   static SOME_STRUCT: &SomeStruct = &SomeStruct {
   |  ___________________________________^
LL | |     foo: &Foo { bools: &[false, true] },
LL | |     bar: &Bar { bools: &[true, true] },
LL | |     f: &id,
LL | |     //~^ ERROR implementation of `FnOnce` is not general enough
LL | | };

error: higher-ranked subtype error
  --> /checkout/src/test/ui/rfc1623.rs:21:35
   |
   |
LL |   static SOME_STRUCT: &SomeStruct = &SomeStruct {
   |  ___________________________________^
LL | |     foo: &Foo { bools: &[false, true] },
LL | |     bar: &Bar { bools: &[true, true] },
LL | |     f: &id,
LL | |     //~^ ERROR implementation of `FnOnce` is not general enough
LL | | };

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 11602 passed; 2 failed; 124 ignored; 0 measured; 0 filtered out; finished in 103.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:19:18
