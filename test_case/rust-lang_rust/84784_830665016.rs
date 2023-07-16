plain
.................................................................................................... 9400/11813
.................................................................................................... 9500/11813
.................................................................................................... 9600/11813
...................................i......i......................................................... 9700/11813
.................................................................................iiiiiii..iiiiii.i.. 9800/11813
................................................................................................F... 9900/11813
.................................................................................................... 10100/11813
.................................................................................................... 10200/11813
.................................................................................................... 10300/11813
.................................................................................................... 10400/11813
---

---- [ui] ui/regions/region-bounds-on-objects-and-type-parameters.rs stdout ----
diff of stderr:

27 LL | struct Foo<'a,'b,'c> {
28    |                  ^^ unused parameter
29    |
- help: if you intended `'c` to be a const parameter, use `const 'c: usize` instead
-   --> $DIR/region-bounds-on-objects-and-type-parameters.rs:11:18
-    |
- LL | struct Foo<'a,'b,'c> {
-    |                  ^^
+    = help: if you intended `'c` to be a const parameter, use `const 'c: usize` instead
35    = help: consider removing `'c`, referring to it in a field, or using a marker such as `PhantomData`
37 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-bounds-on-objects-and-type-parameters/region-bounds-on-objects-and-type-parameters.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-bounds-on-objects-and-type-parameters.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-bounds-on-objects-and-type-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-bounds-on-objects-and-type-parameters" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-bounds-on-objects-and-type-parameters/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0226]: only a single explicit lifetime bound is permitted
   |
   |
LL |     z: Box<dyn Is<'a>+'b+'c>,


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     z: Box<dyn Is<'a>+'b+'c>,
   |
   |
note: lifetime parameter instantiated with the lifetime `'b` as defined on the struct at 11:15
   |
   |
LL | struct Foo<'a,'b,'c> { //~ ERROR parameter `'c` is never used
   |               ^^
note: but lifetime parameter must outlive the lifetime `'a` as defined on the struct at 11:12
   |
   |
LL | struct Foo<'a,'b,'c> { //~ ERROR parameter `'c` is never used


error[E0392]: parameter `'c` is never used
   |
   |
LL | struct Foo<'a,'b,'c> { //~ ERROR parameter `'c` is never used
   |                  ^^ unused parameter
   |
   = help: if you intended `'c` to be a const parameter, use `const 'c: usize` instead
   = help: consider removing `'c`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0226, E0392, E0478.
For more information about an error, try `rustc --explain E0226`.
For more information about an error, try `rustc --explain E0226`.

------------------------------------------


---- [ui] ui/self/self_type_keyword.rs stdout ----
diff of stderr:

77 LL | struct Bar<'Self>;
78    |            ^^^^^ unused parameter
79    |
- help: if you intended `'Self` to be a const parameter, use `const 'Self: usize` instead
-   --> $DIR/self_type_keyword.rs:6:12
-    |
- LL | struct Bar<'Self>;
-    |            ^^^^^
+    = help: if you intended `'Self` to be a const parameter, use `const 'Self: usize` instead
85    = help: consider removing `'Self`, referring to it in a field, or using a marker such as `PhantomData`
87 error: aborting due to 12 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword/self_type_keyword.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/self_type_keyword.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_type_keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:14:13
   |
LL |         ref Self => (),


error: `mut` must be followed by a named binding
   |
   |
LL |         mut Self => (),
   |         ^^^^^^^^ help: remove the `mut` prefix: `Self`
   |
   = note: `mut` may be followed by `variable` and `variable @ pattern`
error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:19:17
   |
   |
LL |         ref mut Self => (),

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:23:15
   |
   |
LL |         Foo { Self } => (),

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:29:26
   |
---

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/self/self_type_keyword.rs:6:12
   |
LL | struct Bar<'Self>;


error: cannot find macro `Self` in this scope
   |
   |
LL |         Self!() => (),


error[E0531]: cannot find unit struct, unit variant or constant `Self` in this scope
   |
   |
LL |         mut Self => (),
   |
help: consider importing this unit struct
   |
LL | use foo::Self;
LL | use foo::Self;
   |

error[E0392]: parameter `'Self` is never used
   |
   |
LL | struct Bar<'Self>;
   |            ^^^^^ unused parameter
   |
   = help: if you intended `'Self` to be a const parameter, use `const 'Self: usize` instead
   = help: consider removing `'Self`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0392, E0531.
For more information about an error, try `rustc --explain E0392`.
For more information about an error, try `rustc --explain E0392`.

------------------------------------------


---- [ui] ui/variance/variance-regions-unused-direct.rs stdout ----
diff of stderr:

4 LL | struct Bivariant<'a>;
5    |                  ^^ unused parameter
6    |
- help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
-   --> $DIR/variance-regions-unused-direct.rs:5:18
-    |
- LL | struct Bivariant<'a>;
-    |                  ^^
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
12    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
13 
14 error[E0392]: parameter `'d` is never used

17 LL | struct Struct<'a, 'd> {
18    |                   ^^ unused parameter
19    |
- help: if you intended `'d` to be a const parameter, use `const 'd: usize` instead
-   --> $DIR/variance-regions-unused-direct.rs:7:19
-    |
- LL | struct Struct<'a, 'd> {
-    |                   ^^
+    = help: if you intended `'d` to be a const parameter, use `const 'd: usize` instead
25    = help: consider removing `'d`, referring to it in a field, or using a marker such as `PhantomData`
27 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct/variance-regions-unused-direct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-regions-unused-direct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-unused-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:5:18
   |
LL | struct Bivariant<'a>; //~ ERROR parameter `'a` is never used
   |                  ^^ unused parameter
   |
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `'d` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:7:19
   |
LL | struct Struct<'a, 'd> { //~ ERROR parameter `'d` is never used
   |                   ^^ unused parameter
   |
   = help: if you intended `'d` to be a const parameter, use `const 'd: usize` instead
   = help: consider removing `'d`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/variance/variance-regions-unused-indirect.rs stdout ----
diff of stderr:

4 LL | enum Foo<'a> {
5    |          ^^ unused parameter
6    |
- help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
-   --> $DIR/variance-regions-unused-indirect.rs:3:10
-    |
- LL | enum Foo<'a> {
-    |          ^^
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
12    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
13 
14 error[E0392]: parameter `'a` is never used

17 LL | enum Bar<'a> {
18    |          ^^ unused parameter
19    |
- help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
-   --> $DIR/variance-regions-unused-indirect.rs:7:10
-    |
- LL | enum Bar<'a> {
-    |          ^^
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
25    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
27 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect/variance-regions-unused-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-regions-unused-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-unused-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:3:10
   |
LL | enum Foo<'a> { //~ ERROR parameter `'a` is never used
   |          ^^ unused parameter
   |
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:7:10
   |
LL | enum Bar<'a> { //~ ERROR parameter `'a` is never used
   |          ^^ unused parameter
   |
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/variance/variance-unused-region-param.rs stdout ----
diff of stderr:

4 LL | struct SomeStruct<'a> { x: u32 }
5    |                   ^^ unused parameter
6    |
- help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
-   --> $DIR/variance-unused-region-param.rs:3:19
-    |
- LL | struct SomeStruct<'a> { x: u32 }
-    |                   ^^
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
12    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
13 
14 error[E0392]: parameter `'a` is never used

17 LL | enum SomeEnum<'a> { Nothing }
18    |               ^^ unused parameter
19    |
- help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
-   --> $DIR/variance-unused-region-param.rs:4:15
-    |
- LL | enum SomeEnum<'a> { Nothing }
-    |               ^^
+    = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
25    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
27 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param/variance-unused-region-param.stderr
To only update this specific test, also pass `--test-args variance/variance-unused-region-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-unused-region-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:3:19
   |
LL | struct SomeStruct<'a> { x: u32 } //~ ERROR parameter `'a` is never used
   |                   ^^ unused parameter
   |
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:4:15
   |
LL | enum SomeEnum<'a> { Nothing } //~ ERROR parameter `'a` is never used
   |               ^^ unused parameter
   |
   = help: if you intended `'a` to be a const parameter, use `const 'a: usize` instead
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/variance/variance-unused-type-param.rs stdout ----
diff of stderr:

4 LL | struct SomeStruct<A> { x: u32 }
5    |                   ^ unused parameter
6    |
- help: if you intended `A` to be a const parameter, use `const A: usize` instead
-   --> $DIR/variance-unused-type-param.rs:6:19
-    |
- LL | struct SomeStruct<A> { x: u32 }
-    |                   ^
+    = help: if you intended `A` to be a const parameter, use `const A: usize` instead
12    = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
13 
14 error[E0392]: parameter `A` is never used

17 LL | enum SomeEnum<A> { Nothing }
18    |               ^ unused parameter
19    |
- help: if you intended `A` to be a const parameter, use `const A: usize` instead
-   --> $DIR/variance-unused-type-param.rs:9:15
-    |
- LL | enum SomeEnum<A> { Nothing }
-    |               ^
+    = help: if you intended `A` to be a const parameter, use `const A: usize` instead
25    = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
27 error[E0392]: parameter `T` is never used


30 LL | enum ListCell<T> {
31    |               ^ unused parameter
32    |
- help: if you intended `T` to be a const parameter, use `const T: usize` instead
-   --> $DIR/variance-unused-type-param.rs:13:15
-    |
- LL | enum ListCell<T> {
-    |               ^
+    = help: if you intended `T` to be a const parameter, use `const T: usize` instead
38    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
40 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/variance-unused-type-param.stderr
To only update this specific test, also pass `--test-args variance/variance-unused-type-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-unused-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:6:19
   |
LL | struct SomeStruct<A> { x: u32 }
   |                   ^ unused parameter
   |
   = help: if you intended `A` to be a const parameter, use `const A: usize` instead
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:9:15
   |
LL | enum SomeEnum<A> { Nothing }
   |               ^ unused parameter
   |
   = help: if you intended `A` to be a const parameter, use `const A: usize` instead
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:13:15
   |
   |
LL | enum ListCell<T> {
   |               ^ unused parameter
   |
   = help: if you intended `T` to be a const parameter, use `const T: usize` instead
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0392`.

---
test result: FAILED. 11710 passed; 6 failed; 97 ignored; 0 measured; 0 filtered out; finished in 125.26s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:03
