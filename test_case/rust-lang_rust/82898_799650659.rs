plain
test [ui (nll)] ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui (nll)] ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs#full_tait stdout ----

28            found opaque type `impl Sized`
29 
29 
30 error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/generic_type_does_not_live_long_enough.rs:12:24
32    |
32    |
- LL | type WrongGeneric<T> = impl 'static;
-    |                        ^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
- ...
36 LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                  - help: consider adding an explicit lifetime bound...: `T: 'static`
+    |
+    |
+    = help: consider adding an explicit lifetime bound `T: 'static`...
38 
39 error[E0310]: the parameter type `T` may not live long enough


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.full_tait.nll/generic_type_does_not_live_long_enough.full_tait.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`

error in revision `full_tait`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.full_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.full_tait.nll/auxiliary"
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
   |                        ------------ the found opaque type
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
For more information about an error, try `rustc --explain E0308`.

------------------------------------------


---- [ui (nll)] ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs#min_tait stdout ----

19            found opaque type `impl Sized`
20 
20 
21 error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/generic_type_does_not_live_long_enough.rs:12:24
23    |
23    |
- LL | type WrongGeneric<T> = impl 'static;
-    |                        ^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
- ...
27 LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
-    |                  - help: consider adding an explicit lifetime bound...: `T: 'static`
+    |
+    |
+    = help: consider adding an explicit lifetime bound `T: 'static`...
29 
30 error[E0310]: the parameter type `T` may not live long enough


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.min_tait.nll/generic_type_does_not_live_long_enough.min_tait.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`

error in revision `min_tait`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.min_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.min_tait.nll/auxiliary"
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
   |                        ------------ the found opaque type
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
For more information about an error, try `rustc --explain E0308`.

------------------------------------------


---- [ui (nll)] ui/type-alias-impl-trait/issue-57611-trait-alias.rs#full_tait stdout ----

7    = note: `#[warn(incomplete_features)]` on by default
8    = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
9 
9 
- error: implementation of `FnOnce` is not general enough
-   --> $DIR/issue-57611-trait-alias.rs:20:16
+ error: higher-ranked subtype error
12    |
12    |
- LL |     type Bar = impl Baz<Self, Self>;
-    |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
-    |
-    = note: closure with signature `fn(&'2 X) -> &X` must implement `FnOnce<(&'1 X,)>`, for any lifetime `'1`...
-    = note: ...but it actually implements `FnOnce<(&'2 X,)>`, for some specific lifetime `'2`
+ LL |         |x| x
18 
- error[E0308]: mismatched types
-   --> $DIR/issue-57611-trait-alias.rs:20:16
-    |
-    |
- LL |     type Bar = impl Baz<Self, Self>;
-    |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
-    |
-    = note: expected type `for<'r> Fn<(&'r X,)>`
-               found type `Fn<(&'<empty> X,)>`
- note: this closure does not fulfill the lifetime requirements
+ error: higher-ranked subtype error
29    |
30 LL |         |x| x

31    |         ^^^^^
31    |         ^^^^^
32 
- error: implementation of `FnOnce` is not general enough
-   --> $DIR/issue-57611-trait-alias.rs:20:16
-    |
- LL |     type Bar = impl Baz<Self, Self>;
-    |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
-    |
-    = note: closure with signature `fn(&'2 X) -> &'2 X` must implement `FnOnce<(&'1 X,)>`, for any lifetime `'1`...
-    = note: ...but it actually implements `FnOnce<(&'2 X,)>`, for some specific lifetime `'2`
42 error[E0308]: mismatched types
43   --> $DIR/issue-57611-trait-alias.rs:20:16
44    |


46    |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
47    |
48    = note: expected type `for<'r> Fn<(&'r X,)>`
-               found type `Fn<(&'<empty> X,)>`
+               found type `Fn<(&'static X,)>`
50 note: this closure does not fulfill the lifetime requirements
52    |


59 LL |     type Bar = impl Baz<Self, Self>;
60    |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
61    |
-    = note: closure with signature `fn(&'2 X) -> &'2 X` must implement `FnOnce<(&'1 X,)>`, for any lifetime `'1`...
-    = note: ...but it actually implements `FnOnce<(&'2 X,)>`, for some specific lifetime `'2`
+    = note: closure with signature `fn(&'static X) -> &'static X` must implement `FnOnce<(&'0 X,)>`, for any lifetime `'0`...
+    = note: ...but it actually implements `FnOnce<(&'static X,)>`
- error: aborting due to 5 previous errors; 1 warning emitted
+ error: aborting due to 4 previous errors; 1 warning emitted
66 
67 For more information about this error, try `rustc --explain E0308`.
67 For more information about this error, try `rustc --explain E0308`.
68 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.full_tait.nll/issue-57611-trait-alias.full_tait.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57611-trait-alias.rs`

error in revision `full_tait`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.full_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.full_tait.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
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


------------------------------------------


---- [ui (nll)] ui/type-alias-impl-trait/issue-57611-trait-alias.rs#min_tait stdout ----


- error: implementation of `FnOnce` is not general enough
-   --> $DIR/issue-57611-trait-alias.rs:20:16
+ error: higher-ranked subtype error
3    |
3    |
- LL |     type Bar = impl Baz<Self, Self>;
-    |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
-    |
-    = note: closure with signature `fn(&'2 X) -> &X` must implement `FnOnce<(&'1 X,)>`, for any lifetime `'1`...
-    = note: ...but it actually implements `FnOnce<(&'2 X,)>`, for some specific lifetime `'2`
+ LL |         |x| x
9 
- error[E0308]: mismatched types
-   --> $DIR/issue-57611-trait-alias.rs:20:16
-    |
-    |
- LL |     type Bar = impl Baz<Self, Self>;
-    |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
-    |
-    = note: expected type `for<'r> Fn<(&'r X,)>`
-               found type `Fn<(&'<empty> X,)>`
- note: this closure does not fulfill the lifetime requirements
+ error: higher-ranked subtype error
20    |
21 LL |         |x| x

22    |         ^^^^^
22    |         ^^^^^
23 
- error: implementation of `FnOnce` is not general enough
-   --> $DIR/issue-57611-trait-alias.rs:20:16
-    |
- LL |     type Bar = impl Baz<Self, Self>;
-    |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
-    |
-    = note: closure with signature `fn(&'2 X) -> &'2 X` must implement `FnOnce<(&'1 X,)>`, for any lifetime `'1`...
-    = note: ...but it actually implements `FnOnce<(&'2 X,)>`, for some specific lifetime `'2`
33 error[E0308]: mismatched types
34   --> $DIR/issue-57611-trait-alias.rs:20:16
35    |


37    |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
38    |
39    = note: expected type `for<'r> Fn<(&'r X,)>`
-               found type `Fn<(&'<empty> X,)>`
+               found type `Fn<(&'static X,)>`
41 note: this closure does not fulfill the lifetime requirements
43    |


50 LL |     type Bar = impl Baz<Self, Self>;
51    |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
52    |
-    = note: closure with signature `fn(&'2 X) -> &'2 X` must implement `FnOnce<(&'1 X,)>`, for any lifetime `'1`...
-    = note: ...but it actually implements `FnOnce<(&'2 X,)>`, for some specific lifetime `'2`
+    = note: closure with signature `fn(&'static X) -> &'static X` must implement `FnOnce<(&'0 X,)>`, for any lifetime `'0`...
+    = note: ...but it actually implements `FnOnce<(&'static X,)>`
- error: aborting due to 5 previous errors
+ error: aborting due to 4 previous errors
57 
58 For more information about this error, try `rustc --explain E0308`.
58 For more information about this error, try `rustc --explain E0308`.
59 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.min_tait.nll/issue-57611-trait-alias.min_tait.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57611-trait-alias.rs`

error in revision `min_tait`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.min_tait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.min_tait.nll/auxiliary"
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
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 11551 passed; 4 failed; 124 ignored; 0 measured; 0 filtered out; finished in 103.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:18:50
