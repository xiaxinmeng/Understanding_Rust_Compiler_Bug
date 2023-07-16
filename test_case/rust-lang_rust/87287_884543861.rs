plain
1 error: lifetime may not live long enough
-   --> $DIR/ret-impl-trait-one.rs:10:80
+   --> $DIR/ret-impl-trait-one.rs:10:65
3    |
- LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
-    |  ________________________________--__--__________________________________________^
-    | |                                |   |
-    | |                                |   lifetime `'b` defined here
-    | |                                lifetime `'a` defined here
- LL | |
- LL | |     (a, b)
- LL | | }
-    | |_^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+ LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
+    |                                --  --                           ^^^^^^^^^^^^^^ opaque type requires that `'b` must outlive `'a`
+    |                                |   |
+    |                                |   lifetime `'b` defined here
+    |                                lifetime `'a` defined here
13    |
14    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll/ret-impl-trait-one.nll.stderr
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs:10:65
   |
LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |                                --  --                           ^^^^^^^^^^^^^^ opaque type requires that `'b` must outlive `'a`
   |                                |   |
   |                                |   lifetime `'b` defined here
   |                                lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error


------------------------------------------
---
1 error: lifetime may not live long enough
-   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait-async.rs:8:48
+   --> $DIR/arbitrary_self_types_pin_lifetime_impl_trait-async.rs:8:37
3    |
4 LL |     async fn f(self: Pin<&Self>) -> impl Clone { self }
-    |                          -                     ^^^^^^^^ returning this value requires that `'1` must outlive `'static`
+    |                          -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
6    |                          |
7    |                          let's call the lifetime of this reference `'1`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll/arbitrary_self_types_pin_lifetime_impl_trait-async.nll.stderr
To only update this specific test, also pass `--test-args self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime_impl_trait-async.rs:8:37
   |
LL |     async fn f(self: Pin<&Self>) -> impl Clone { self }
   |                          -          ^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
   |                          |
   |                          let's call the lifetime of this reference `'1`
   |
help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
   |
LL |     async fn f(self: Pin<&Self>) -> impl Clone + '_ { self }

error: aborting due to previous error


---
diff of stderr:

28            found opaque type `impl Sized`
29 
30 error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/generic_type_does_not_live_long_enough.rs:17:30
32    |
32    |
33 LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.full_tait.nll/generic_type_does_not_live_long_enough.full_tait.nll.stderr
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`


error in revision `full_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.full_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.full_tait.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: at least one trait must be specified
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:12:24
   |
LL | type WrongGeneric<T> = impl 'static;

warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:3:32
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information


error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:9:18
   |
LL |     let z: i32 = x; //~ ERROR mismatched types
   |            ---   ^ expected `i32`, found opaque type
   |            expected due to this
...
...
LL | type WrongGeneric<T> = impl 'static;
   |
   = note:     expected type `i32`
           found opaque type `impl Sized`


error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...

error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL | type WrongGeneric<T> = impl 'static;
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...
   = note: ...so that the type `T` will meet its required lifetime bounds
error: aborting due to 4 previous errors; 1 warning emitted

Some errors have detailed explanations: E0308, E0310.
For more information about an error, try `rustc --explain E0308`.
---
diff of stderr:

19            found opaque type `impl Sized`
20 
21 error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/generic_type_does_not_live_long_enough.rs:17:30
23    |
23    |
24 LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.min_tait.nll/generic_type_does_not_live_long_enough.min_tait.nll.stderr
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`


error in revision `min_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.min_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.min_tait.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: at least one trait must be specified
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:12:24
   |
LL | type WrongGeneric<T> = impl 'static;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:9:18
   |
   |
LL |     let z: i32 = x; //~ ERROR mismatched types
   |            ---   ^ expected `i32`, found opaque type
   |            expected due to this
...
...
LL | type WrongGeneric<T> = impl 'static;
   |
   = note:     expected type `i32`
           found opaque type `impl Sized`


error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...

error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL | type WrongGeneric<T> = impl 'static;
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...
   = note: ...so that the type `T` will meet its required lifetime bounds
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0310.
For more information about an error, try `rustc --explain E0308`.
---
9    |
10 LL |         |x| x
11    |         ^^^^^

19    = note: expected type `for<'r> Fn<(&'r X,)>`
20               found type `Fn<(&'static X,)>`
21 note: this closure does not fulfill the lifetime requirements
-   --> $DIR/issue-57611-trait-alias.rs:28:9
23    |
24 LL |         |x| x
25    |         ^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.min_tait.nll/issue-57611-trait-alias.min_tait.nll.stderr
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57611-trait-alias.rs`


error in revision `min_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.min_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.min_tait.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:20:16
   |
LL |     type Bar = impl Baz<Self, Self>;
   |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `for<'r> Fn<(&'r X,)>`
              found type `Fn<(&'static X,)>`
note: this closure does not fulfill the lifetime requirements
   |
   |
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |         |x| x


error: implementation of `FnOnce` is not general enough
   |
   |
LL |     type Bar = impl Baz<Self, Self>;
   |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'static X) -> &'static X` must implement `FnOnce<(&'0 X,)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(&'static X,)>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.

---
18    |
19 LL |         |x| x
20    |         ^^^^^

28    = note: expected type `for<'r> Fn<(&'r X,)>`
29               found type `Fn<(&'static X,)>`
30 note: this closure does not fulfill the lifetime requirements
-   --> $DIR/issue-57611-trait-alias.rs:28:9
32    |
33 LL |         |x| x
34    |         ^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.full_tait.nll/issue-57611-trait-alias.full_tait.nll.stderr
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57611-trait-alias.rs`


error in revision `full_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.full_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.full_tait.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:8:32
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information

---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:20:16
   |
LL |     type Bar = impl Baz<Self, Self>;
   |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected type `for<'r> Fn<(&'r X,)>`
              found type `Fn<(&'static X,)>`
note: this closure does not fulfill the lifetime requirements
   |
LL |         |x| x
   |         ^^^^^


error: implementation of `FnOnce` is not general enough
   |
   |
LL |     type Bar = impl Baz<Self, Self>;
   |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'static X) -> &'static X` must implement `FnOnce<(&'0 X,)>`, for any lifetime `'0`...
   = note: ...but it actually implements `FnOnce<(&'static X,)>`
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 12020 passed; 6 failed; 125 ignored; 0 measured; 0 filtered out; finished in 87.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:16:28
