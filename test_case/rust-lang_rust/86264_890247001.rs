plain
test [ui] ui/traits/suggest-deferences/multiple-0.rs ... ok
test [ui] ui/traits/syntax-polarity.rs ... ok
test [ui] ui/traits/wf-object/maybe-bound.rs ... ok
test [ui] ui/traits/wf-object/no-duplicates.rs ... ok
test [ui] ui/traits/trait-upcasting/multiple-occurence-ambiguousity.rs ... ok
test [ui] ui/traits/trait-upcasting/type-checking-test-1.rs ... ok
test [ui] ui/traits/wf-object/only-maybe-bound.rs ... ok
test [ui] ui/traits/wf-object/only-maybe-bound.rs ... ok
test [ui] ui/traits/trait-upcasting/issue-11515-upcast-fn_mut-fn.rs ... ok
test [ui] ui/traits/trait-upcasting/type-checking-test-2.rs ... ok
test [ui] ui/traits/vtable/vtable-multiple.rs ... ok
test [ui] ui/traits/ufcs-object.rs ... ok
test [ui] ui/traits/use-before-def.rs ... ok
test [ui] ui/traits/vtable/vtable-vacant.rs ... ok
---
test [ui (nll)] ui/traits/wf-object/maybe-bound.rs ... ok
test [ui (nll)] ui/traits/superdefault-generics.rs ... ok
test [ui (nll)] ui/traits/wf-object/no-duplicates.rs ... ok
test [ui (nll)] ui/traits/wf-object/only-maybe-bound.rs ... ok
test [ui (nll)] ui/traits/trait-upcasting/multiple-occurence-ambiguousity.rs ... ok
test [ui (nll)] ui/traits/trait-upcasting/type-checking-test-1.rs ... ok
test [ui (nll)] ui/traits/use-before-def.rs ... ok
test [ui (nll)] ui/traits/trait-upcasting/type-checking-test-2.rs ... ok
test [ui (nll)] ui/traits/trait-upcasting/type-checking-test-2.rs ... ok
test [ui (nll)] ui/traits/trait-upcasting/issue-11515-upcast-fn_mut-fn.rs ... ok
test [ui (nll)] ui/traits/vtable/vtable-multiple.rs ... ok
test [ui (nll)] ui/traits/vtable/vtable-multi-level.rs ... ok
test [ui (nll)] ui/transmute-equal-assoc-types.rs ... ok
test [ui (nll)] ui/traits/vtable/vtable-vacant.rs ... ok
---
test [ui (nll)] ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui (nll)] ui/traits/trait-upcasting/type-checking-test-3.rs stdout ----

- error[E0308]: mismatched types
+ error: lifetime may not live long enough
2   --> $DIR/type-checking-test-3.rs:12:13
2   --> $DIR/type-checking-test-3.rs:12:13
3    |
+ LL | fn test_wrong1<'a>(x: &dyn Foo<'static>, y: &'a u32) {
+    |                -- lifetime `'a` defined here
4 LL |     let _ = x as &dyn Bar<'a>; // Error
-    |             ^ lifetime mismatch
+    |             ^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
6    |
-    = note: expected trait object `dyn Bar<'a>`
-               found trait object `dyn Bar<'static>`
- note: the lifetime `'a` as defined on the function body at 11:16...
-   --> $DIR/type-checking-test-3.rs:11:16
-    |
- LL | fn test_wrong1<'a>(x: &dyn Foo<'static>, y: &'a u32) {
-    |                ^^
-    = note: ...does not necessarily outlive the static lifetime
+    = help: consider replacing `'a` with `'static`
- error[E0308]: mismatched types
+ error: lifetime may not live long enough
17   --> $DIR/type-checking-test-3.rs:17:13
18    |
18    |
+ LL | fn test_wrong2<'a>(x: &dyn Foo<'a>) {
+    |                -- lifetime `'a` defined here
19 LL |     let _ = x as &dyn Bar<'static>; // Error
-    |             ^ lifetime mismatch
+    |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
21    |
-    = note: expected trait object `dyn Bar<'static>`
-               found trait object `dyn Bar<'a>`
- note: the lifetime `'a` as defined on the function body at 16:16...
-   --> $DIR/type-checking-test-3.rs:16:16
-    |
- LL | fn test_wrong2<'a>(x: &dyn Foo<'a>) {
-    |                ^^
-    = note: ...does not necessarily outlive the static lifetime
+    = help: consider replacing `'a` with `'static`
31 error: aborting due to 2 previous errors
32 

- For more information about this error, try `rustc --explain E0308`.
- For more information about this error, try `rustc --explain E0308`.
34 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/type-checking-test-3.nll/type-checking-test-3.nll.stderr
To only update this specific test, also pass `--test-args traits/trait-upcasting/type-checking-test-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/type-checking-test-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/type-checking-test-3.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/type-checking-test-3.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/traits/trait-upcasting/type-checking-test-3.rs:12:13
   |
LL | fn test_wrong1<'a>(x: &dyn Foo<'static>, y: &'a u32) {
   |                -- lifetime `'a` defined here
LL |     let _ = x as &dyn Bar<'a>; // Error
   |             ^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
   |
   = help: consider replacing `'a` with `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/traits/trait-upcasting/type-checking-test-3.rs:17:13
   |
   |
LL | fn test_wrong2<'a>(x: &dyn Foo<'a>) {
   |                -- lifetime `'a` defined here
LL |     let _ = x as &dyn Bar<'static>; // Error
   |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui (nll)] ui/traits/trait-upcasting/type-checking-test-4.rs stdout ----

- error[E0308]: mismatched types
+ error: lifetime may not live long enough
2   --> $DIR/type-checking-test-4.rs:16:13
2   --> $DIR/type-checking-test-4.rs:16:13
3    |
+ LL | fn test_wrong1<'a>(x: &dyn Foo<'static>, y: &'a u32) {
+    |                -- lifetime `'a` defined here
4 LL |     let _ = x as &dyn Bar<'static, 'a>; // Error
-    |             ^ lifetime mismatch
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
6    |
-    = note: expected trait object `dyn Bar<'static, 'a>`
-               found trait object `dyn Bar<'static, 'static>`
- note: the lifetime `'a` as defined on the function body at 15:16...
-   --> $DIR/type-checking-test-4.rs:15:16
-    |
- LL | fn test_wrong1<'a>(x: &dyn Foo<'static>, y: &'a u32) {
-    |                ^^
-    = note: ...does not necessarily outlive the static lifetime
+    = help: consider replacing `'a` with `'static`
- error[E0308]: mismatched types
+ error: lifetime may not live long enough
17   --> $DIR/type-checking-test-4.rs:21:13
18    |
18    |
+ LL | fn test_wrong2<'a>(x: &dyn Foo<'static>, y: &'a u32) {
+    |                -- lifetime `'a` defined here
19 LL |     let _ = x as &dyn Bar<'a, 'static>; // Error
-    |             ^ lifetime mismatch
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
21    |
-    = note: expected trait object `dyn Bar<'a, 'static>`
-               found trait object `dyn Bar<'static, 'static>`
- note: the lifetime `'a` as defined on the function body at 20:16...
-   --> $DIR/type-checking-test-4.rs:20:16
-    |
- LL | fn test_wrong2<'a>(x: &dyn Foo<'static>, y: &'a u32) {
-    |                ^^
-    = note: ...does not necessarily outlive the static lifetime
+    = help: consider replacing `'a` with `'static`
30 
- error[E0759]: `x` has lifetime `'a` but it needs to satisfy a `'static` lifetime requirement
-   --> $DIR/type-checking-test-4.rs:26:27
+ error: lifetime may not live long enough
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
33    |
33    |
34 LL | fn test_wrong3<'a>(x: &dyn Foo<'a>) -> Option<&'static u32> {
-    |                       ------------ this data with lifetime `'a`...
- LL |     let y = x as &dyn Bar<'_, '_>;
-    |             -             ^^
-    |             |
-    |             ...is captured here...
- LL |
+    |                -- lifetime `'a` defined here
+ ...
41 LL |     y.get_b() // ERROR
-    |     --------- ...and is required to live as long as `'static` here
+    |     ^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
+    |
+    = help: consider replacing `'a` with `'static`
44 error: aborting due to 3 previous errors
45 

- Some errors have detailed explanations: E0308, E0759.
- Some errors have detailed explanations: E0308, E0759.
- For more information about an error, try `rustc --explain E0308`.
48 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/type-checking-test-4.nll/type-checking-test-4.nll.stderr
To only update this specific test, also pass `--test-args traits/trait-upcasting/type-checking-test-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-upcasting/type-checking-test-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/type-checking-test-4.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-upcasting/type-checking-test-4.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/traits/trait-upcasting/type-checking-test-4.rs:16:13
   |
LL | fn test_wrong1<'a>(x: &dyn Foo<'static>, y: &'a u32) {
   |                -- lifetime `'a` defined here
LL |     let _ = x as &dyn Bar<'static, 'a>; // Error
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
   |
   = help: consider replacing `'a` with `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/traits/trait-upcasting/type-checking-test-4.rs:21:13
   |
   |
LL | fn test_wrong2<'a>(x: &dyn Foo<'static>, y: &'a u32) {
   |                -- lifetime `'a` defined here
LL |     let _ = x as &dyn Bar<'a, 'static>; // Error
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
   |
   = help: consider replacing `'a` with `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/traits/trait-upcasting/type-checking-test-4.rs:28:5
   |
   |
LL | fn test_wrong3<'a>(x: &dyn Foo<'a>) -> Option<&'static u32> {
   |                -- lifetime `'a` defined here
...
LL |     y.get_b() // ERROR
   |     ^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to 3 previous errors


------------------------------------------
---
test result: FAILED. 11959 passed; 2 failed; 126 ignored; 0 measured; 0 filtered out; finished in 84.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:15:56
