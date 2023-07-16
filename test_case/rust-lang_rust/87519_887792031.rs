plain
diff of stderr:

19            found opaque type `impl Sized`
20 
21 error[E0310]: the parameter type `T` may not live long enough
+    |
+    |
+ LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
+    |
+    |
+    = help: consider adding an explicit lifetime bound `T: 'static`...
+ 
+ error[E0310]: the parameter type `T` may not live long enough
23    |
23    |
24 LL | type WrongGeneric<T> = impl 'static;

-    |                        ^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
- ...
- LL | fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
-    |                  - help: consider adding an explicit lifetime bound...: `T: 'static`
+    |
+    |
+    = help: consider adding an explicit lifetime bound `T: 'static`...
+    = note: ...so that the type `T` will meet its required lifetime bounds
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
31 
32 Some errors have detailed explanations: E0308, E0310.
32 Some errors have detailed explanations: E0308, E0310.
33 For more information about an error, try `rustc --explain E0308`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/generic_type_does_not_live_long_enough.nll.stderr
To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: at least one trait must be specified
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:9:24
   |
LL | type WrongGeneric<T> = impl 'static;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:6:18
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
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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

+ error: higher-ranked subtype error
+   --> $DIR/issue-57611-trait-alias.rs:21:9
+    |
+ LL |         |x| x
+ 
+ error: higher-ranked subtype error
+   --> $DIR/issue-57611-trait-alias.rs:21:9
+    |
+    |
+ LL |         |x| x
+ 
+ error[E0308]: mismatched types
+   --> $DIR/issue-57611-trait-alias.rs:17:16
+    |
+    |
+ LL |     type Bar = impl Baz<Self, Self>;
+    |                ^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
+    |
+    = note: expected type `for<'r> Fn<(&'r X,)>`
+               found type `Fn<(&'static X,)>`
+ note: this closure does not fulfill the lifetime requirements
+    |
+    |
+ LL |         |x| x
+ 
+ 
1 error: implementation of `FnOnce` is not general enough
3    |


4 LL |     type Bar = impl Baz<Self, Self>;
5    |                ^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
6    |
-    = note: closure with signature `fn(&'2 X) -> &X` must implement `FnOnce<(&'1 X,)>`, for any lifetime `'1`...
-    = note: ...but it actually implements `FnOnce<(&'2 X,)>`, for some specific lifetime `'2`
+    = note: closure with signature `fn(&'static X) -> &'static X` must implement `FnOnce<(&'0 X,)>`, for any lifetime `'0`...
+    = note: ...but it actually implements `FnOnce<(&'static X,)>`
- error: aborting due to previous error
+ error: aborting due to 4 previous errors
11 
+ For more information about this error, try `rustc --explain E0308`.
+ For more information about this error, try `rustc --explain E0308`.
12 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.nll/issue-57611-trait-alias.nll.stderr
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-57611-trait-alias.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-57611-trait-alias.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-57611-trait-alias.rs:17:16
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
test result: FAILED. 11941 passed; 2 failed; 123 ignored; 0 measured; 0 filtered out; finished in 107.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:21:04
