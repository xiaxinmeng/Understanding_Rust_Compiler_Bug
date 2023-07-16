plain
1 error: lifetime may not live long enough
-   --> $DIR/regions-close-object-into-object-2.rs:10:5
+   --> $DIR/regions-close-object-into-object-2.rs:9:5
3    |
4 LL | fn g<'a, T: 'static>(v: Box<dyn A<T> + 'a>) -> Box<dyn X + 'static> {
5    |      -- lifetime `'a` defined here

- LL |     box B(&*v) as Box<dyn X>
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
8    |
9    = help: consider replacing `'a` with `'static`


11 error[E0515]: cannot return value referencing local data `*v`
-   --> $DIR/regions-close-object-into-object-2.rs:10:5
13    |
13    |
- LL |     box B(&*v) as Box<dyn X>
-    |     ^^^^^^---^^^^^^^^^^^^^^^
-    |     |     |
-    |     |     `*v` is borrowed here
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |     ^^^^^^^^^^^---^^^^^^^^^^^^^^^^
+    |     |          |
+    |     |          `*v` is borrowed here
18    |     returns a value referencing data owned by the current function
20 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2.nll/regions-close-object-into-object-2.nll.stderr
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-2.rs:9:5
   |
LL | fn g<'a, T: 'static>(v: Box<dyn A<T> + 'a>) -> Box<dyn X + 'static> {
   |      -- lifetime `'a` defined here
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
   |
   = help: consider replacing `'a` with `'static`

error[E0515]: cannot return value referencing local data `*v`
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |     ^^^^^^^^^^^---^^^^^^^^^^^^^^^^
   |     |          |
   |     |          `*v` is borrowed here
   |     returns a value referencing data owned by the current function
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0515`.


------------------------------------------


---- [ui (nll)] ui/regions/regions-close-object-into-object-4.rs stdout ----
diff of stderr:

1 error[E0310]: the parameter type `U` may not live long enough
-   --> $DIR/regions-close-object-into-object-4.rs:10:5
3    |
3    |
- LL |     box B(&*v) as Box<dyn X>
-    |     ^^^^^^^^^^
+ LL |     Box::new(B(&*v)) as Box<dyn X>
6    |
6    |
7    = help: consider adding an explicit lifetime bound `U: 'static`...


+ error[E0310]: the parameter type `U` may not live long enough
+    |
+    |
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |
+    |
+    = help: consider adding an explicit lifetime bound `U: 'static`...
+ 
+ error[E0310]: the parameter type `U` may not live long enough
+    |
+    |
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |
+    |
+    = help: consider adding an explicit lifetime bound `U: 'static`...
9 error: lifetime may not live long enough
-   --> $DIR/regions-close-object-into-object-4.rs:10:5
+   --> $DIR/regions-close-object-into-object-4.rs:9:5
11    |
11    |
12 LL | fn i<'a, T, U>(v: Box<dyn A<U>+'a>) -> Box<dyn X + 'static> {
13    |      -- lifetime `'a` defined here

- LL |     box B(&*v) as Box<dyn X>
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
16    |
17    = help: consider replacing `'a` with `'static`


19 error[E0515]: cannot return value referencing local data `*v`
-   --> $DIR/regions-close-object-into-object-4.rs:10:5
21    |
21    |
- LL |     box B(&*v) as Box<dyn X>
-    |     ^^^^^^---^^^^^^^^^^^^^^^
-    |     |     |
-    |     |     `*v` is borrowed here
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |     ^^^^^^^^^^^---^^^^^^^^^^^^^^^^
+    |     |          |
+    |     |          `*v` is borrowed here
26    |     returns a value referencing data owned by the current function
27 
28 error[E0310]: the parameter type `U` may not live long enough
-   --> $DIR/regions-close-object-into-object-4.rs:10:9
+   --> $DIR/regions-close-object-into-object-4.rs:9:14
30    |
30    |
- LL |     box B(&*v) as Box<dyn X>
-    |         ^^^^^^
+ LL |     Box::new(B(&*v)) as Box<dyn X>
33    |
33    |
34    = help: consider adding an explicit lifetime bound `U: 'static`...

- error: aborting due to 4 previous errors
+ error: aborting due to 6 previous errors
37 
37 
38 Some errors have detailed explanations: E0310, E0515.
39 For more information about an error, try `rustc --explain E0310`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4.nll/regions-close-object-into-object-4.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0310]: the parameter type `U` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |
   |
   = help: consider adding an explicit lifetime bound `U: 'static`...

error[E0310]: the parameter type `U` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |
   |
   = help: consider adding an explicit lifetime bound `U: 'static`...

error[E0310]: the parameter type `U` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |
   |
   = help: consider adding an explicit lifetime bound `U: 'static`...
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-4.rs:9:5
   |
   |
LL | fn i<'a, T, U>(v: Box<dyn A<U>+'a>) -> Box<dyn X + 'static> {
   |      -- lifetime `'a` defined here
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
   |
   = help: consider replacing `'a` with `'static`

error[E0515]: cannot return value referencing local data `*v`
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |     ^^^^^^^^^^^---^^^^^^^^^^^^^^^^
   |     |          |
   |     |          `*v` is borrowed here
   |     returns a value referencing data owned by the current function

error[E0310]: the parameter type `U` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X> //~ ERROR E0759
   |
   |
   = help: consider adding an explicit lifetime bound `U: 'static`...
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0310, E0515.
For more information about an error, try `rustc --explain E0310`.
For more information about an error, try `rustc --explain E0310`.

------------------------------------------


---- [ui (nll)] ui/regions/regions-close-object-into-object-5.rs stdout ----
diff of stderr:

1 error[E0310]: the parameter type `T` may not live long enough
3    |
3    |
- LL |     box B(&*v) as Box<X>
-    |     ^^^^^^^^^^
+ LL |     Box::new(B(&*v)) as Box<dyn X>
6    |
6    |
7    = help: consider adding an explicit lifetime bound `T: 'static`...


+ error[E0310]: the parameter type `T` may not live long enough
+    |
+    |
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |
+    |
+    = help: consider adding an explicit lifetime bound `T: 'static`...
+ 
+ error[E0310]: the parameter type `T` may not live long enough
+    |
+    |
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |
+    |
+    = help: consider adding an explicit lifetime bound `T: 'static`...
+ 
9 error[E0515]: cannot return value referencing local data `*v`
11    |


- LL |     box B(&*v) as Box<X>
-    |     ^^^^^^---^^^^^^^^^^^
-    |     |     |
-    |     |     `*v` is borrowed here
+ LL |     Box::new(B(&*v)) as Box<dyn X>
+    |     ^^^^^^^^^^^---^^^^^^^^^^^^^^^^
+    |     |          |
+    |     |          `*v` is borrowed here
16    |     returns a value referencing data owned by the current function
17 
18 error[E0310]: the parameter type `T` may not live long enough
-   --> $DIR/regions-close-object-into-object-5.rs:17:9
+   --> $DIR/regions-close-object-into-object-5.rs:17:14
20    |
20    |
- LL |     box B(&*v) as Box<X>
-    |         ^^^^^^
+ LL |     Box::new(B(&*v)) as Box<dyn X>
23    |
23    |
24    = help: consider adding an explicit lifetime bound `T: 'static`...

- error: aborting due to 3 previous errors
+ error: aborting due to 5 previous errors
27 
27 
28 Some errors have detailed explanations: E0310, E0515.
29 For more information about an error, try `rustc --explain E0310`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5.nll/regions-close-object-into-object-5.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...

error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...

error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...

error[E0515]: cannot return value referencing local data `*v`
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^^^^---^^^^^^^^^^^^^^^^
   |     |          |
   |     |          `*v` is borrowed here
   |     returns a value referencing data owned by the current function

error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     Box::new(B(&*v)) as Box<dyn X>
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0310, E0515.
For more information about an error, try `rustc --explain E0310`.
For more information about an error, try `rustc --explain E0310`.

------------------------------------------


---- [ui (nll)] ui/regions/regions-close-over-type-parameter-1.rs stdout ----
diff of stderr:

1 error[E0310]: the parameter type `A` may not live long enough
3    |
3    |
- LL |     box v as Box<dyn SomeTrait + 'static>
-    |     ^^^^^
+ LL |     Box::new(v) as Box<dyn SomeTrait + 'static>
6    |
6    |
7    = help: consider adding an explicit lifetime bound `A: 'static`...


9 error[E0309]: the parameter type `A` may not live long enough
11    |
11    |
- LL |     box v as Box<dyn SomeTrait + 'b>
-    |     ^^^^^
+ LL |     Box::new(v) as Box<dyn SomeTrait + 'b>
14    |
14    |
15    = help: consider adding an explicit lifetime bound `A: 'b`...


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-1.nll/regions-close-over-type-parameter-1.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-close-over-type-parameter-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-over-type-parameter-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-1.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-1.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0310]: the parameter type `A` may not live long enough
   |
   |
LL |     Box::new(v) as Box<dyn SomeTrait + 'static>
   |
   |
   = help: consider adding an explicit lifetime bound `A: 'static`...

error[E0309]: the parameter type `A` may not live long enough
   |
   |
LL |     Box::new(v) as Box<dyn SomeTrait + 'b>
   |
   |
   = help: consider adding an explicit lifetime bound `A: 'b`...
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0309, E0310.
For more information about an error, try `rustc --explain E0309`.
---
---- [ui (nll)] ui/regions/regions-close-over-type-parameter-multiple.rs stdout ----
diff of stderr:

6    |                    |
7    |                    lifetime `'a` defined here
8 LL |     // A outlives 'a AND 'b...but not 'c.
- LL |     box v as Box<dyn SomeTrait + 'a>
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'c`
+ LL |     Box::new(v) as Box<dyn SomeTrait + 'a>
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'c`
11    |
12    = help: consider adding the following bound: `'a: 'c`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-multiple.nll/regions-close-over-type-parameter-multiple.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-close-over-type-parameter-multiple.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-over-type-parameter-multiple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-multiple.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-multiple.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-over-type-parameter-multiple.rs:20:5
   |
LL | fn make_object_bad<'a,'b,'c,A:SomeTrait+'a+'b>(v: A) -> Box<dyn SomeTrait + 'c> {
   |                    --    -- lifetime `'c` defined here
   |                    |
   |                    lifetime `'a` defined here
LL |     // A outlives 'a AND 'b...but not 'c.
LL |     Box::new(v) as Box<dyn SomeTrait + 'a> //~ ERROR cannot infer an appropriate lifetime
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'c`
   |
   = help: consider adding the following bound: `'a: 'c`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12018 passed; 5 failed; 128 ignored; 0 measured; 0 filtered out; finished in 100.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:19:56
