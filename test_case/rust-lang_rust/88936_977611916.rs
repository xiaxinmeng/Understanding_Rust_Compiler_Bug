plain

---- [ui (nll)] ui/kindck/kindck-send-object1.rs stdout ----
diff of stderr:

5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
6    |
7    = help: the trait `Sync` is not implemented for `(dyn Dummy + 'a)`
+    = note: consider using `std::sync::Arc<(dyn Dummy + 'a)>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
8    = note: required because of the requirements on the impl of `Send` for `&'a (dyn Dummy + 'a)`
9 note: required by a bound in `assert_send`
10   --> $DIR/kindck-send-object1.rs:5:18

19    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
20    |
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
21    = help: the trait `Send` is not implemented for `(dyn Dummy + 'a)`
+    = note: consider using `std::sync::Arc<(dyn Dummy + 'a)>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
22    = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Dummy + 'a)>`
23    = note: required because it appears within the type `Box<(dyn Dummy + 'a)>`
24 note: required by a bound in `assert_send`

The actual stderr differed from the expected stderr.
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
   = note: consider using `std::sync::Arc<(dyn Dummy + 'a)>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
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
   = note: consider using `std::sync::Arc<(dyn Dummy + 'a)>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
   = note: required because of the requirements on the impl of `Send` for `Unique<(dyn Dummy + 'a)>`
   = note: required because it appears within the type `Box<(dyn Dummy + 'a)>`
note: required by a bound in `assert_send`
  --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:5:18
   |
LL | fn assert_send<T:Send+'static>() { }
   |                  ^^^^ required by this bound in `assert_send`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui (nll)] ui/kindck/kindck-impl-type-params.rs stdout ----
diff of stderr:

4 LL |     let a = &t as &dyn Gettable<T>;
5    |             ^^ `T` cannot be sent between threads safely
6    |
+    = note: consider using `std::sync::Arc<T>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
7 note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
8   --> $DIR/kindck-impl-type-params.rs:14:32


38 LL |     let a: &dyn Gettable<T> = &t;
39    |                               ^^ `T` cannot be sent between threads safely
40    |
+    = note: consider using `std::sync::Arc<T>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
41 note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
42   --> $DIR/kindck-impl-type-params.rs:14:32


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/kindck-impl-type-params.nll.stderr
To only update this specific test, also pass `--test-args kindck/kindck-impl-type-params.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-impl-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/auxiliary"
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
   = note: consider using `std::sync::Arc<T>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
LL | fn f<T: std::marker::Send>(val: T) {


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
LL | fn f<T: std::marker::Copy>(val: T) {


error[E0277]: `T` cannot be sent between threads safely
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
   |
LL |     let a: &dyn Gettable<T> = &t;
   |                               ^^ `T` cannot be sent between threads safely
   |
   = note: consider using `std::sync::Arc<T>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
LL | fn g<T: std::marker::Send>(val: T) {


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
LL | fn g<T: std::marker::Copy>(val: T) {

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

10    | |__^ `dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>` cannot be shared between threads safely
11    |
12    = help: within `&SomeStruct`, the trait `Sync` is not implemented for `dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>`
+    = note: consider using `std::sync::Arc<dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
13    = note: required because it appears within the type `&dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>`
14 note: required because it appears within the type `SomeStruct`
15   --> $DIR/rfc1623.rs:11:8

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623.nll/rfc1623.nll.stderr
To only update this specific test, also pass `--test-args rfc1623.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1623.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623.nll/auxiliary"
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
   = note: consider using `std::sync::Arc<dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>>`; for more information visit <https://doc.rust-lang.org/book/ch16-03-shared-state.html>
   = note: required because it appears within the type `&dyn for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>`
note: required because it appears within the type `SomeStruct`
   |
   |
LL | struct SomeStruct<'x, 'y, 'z: 'x> {
   |        ^^^^^^^^^^
   = note: required because it appears within the type `&SomeStruct`
   = note: shared static variables must have a type that implements `Sync`
error[E0308]: mismatched types
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
   | |_^ one type is more general than the other
   |
   = note: expected type `for<'a, 'b> Fn<(&'a Foo<'b>,)>`
              found type `Fn<(&Foo<'_>,)>`
error[E0308]: mismatched types
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
   | |_^ one type is more general than the other
   |
   = note: expected type `for<'a, 'b> Fn<(&'a Foo<'b>,)>`
              found type `Fn<(&Foo<'_>,)>`

error: implementation of `FnOnce` is not general enough
   |
   |
LL |   static SOME_STRUCT: &SomeStruct = &SomeStruct {
   |  ___________________________________^
LL | |     foo: &Foo { bools: &[false, true] },
LL | |     bar: &Bar { bools: &[true, true] },
LL | |     f: &id,
LL | |     //~^ ERROR implementation of `FnOnce` is not general enough
LL | | };
   | |_^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&'2 Foo<'_>) -> &'2 Foo<'_> {id::<&'2 Foo<'_>>}` must implement `FnOnce<(&'1 Foo<'b>,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 Foo<'_>,)>`, for some specific lifetime `'2`

error: implementation of `FnOnce` is not general enough
   |
   |
LL |   static SOME_STRUCT: &SomeStruct = &SomeStruct {
   |  ___________________________________^
LL | |     foo: &Foo { bools: &[false, true] },
LL | |     bar: &Bar { bools: &[true, true] },
LL | |     f: &id,
LL | |     //~^ ERROR implementation of `FnOnce` is not general enough
LL | | };
   | |_^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&Foo<'2>) -> &Foo<'2> {id::<&Foo<'2>>}` must implement `FnOnce<(&'a Foo<'1>,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&Foo<'2>,)>`, for some specific lifetime `'2`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
---
test result: FAILED. 12257 passed; 3 failed; 146 ignored; 0 measured; 0 filtered out; finished in 111.42s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:18:45
