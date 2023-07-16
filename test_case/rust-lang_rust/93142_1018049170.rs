plain
..............................................i..................................................... 2900/12540
.................................................................................................... 3000/12540
.............................................................iiiii.................................. 3100/12540
.................................................................................................... 3200/12540
................................F.......F........................................................... 3300/12540
.................................................................................................... 3500/12540
.................................................................................................... 3600/12540
...i.........i.........i............................................................................ 3700/12540
.............................................................................i...................... 3800/12540
---
1 error[E0601]: `main` function not found in crate `imported_main_const_forbidden`
-   --> $DIR/imported_main_const_forbidden.rs:1:1
+   --> $DIR/imported_main_const_forbidden.rs:7:22
3    |
- LL | / #![feature(imported_main)]
- LL | |
- LL | | pub mod foo {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | |     pub const BAR: usize = 42;
- LL | | }
- LL | |
- LL | | use foo::BAR as main;
-    | |_____----------------^ consider adding a `main` function to `$DIR/imported_main_const_forbidden.rs`
-    |       |
-    |       non-function item at `crate::main` is found
+ LL | use foo::BAR as main;
+    |     ---------------- ^ consider adding a `main` function to `$DIR/imported_main_const_forbidden.rs`
+    |     |
+    |     non-function item at `crate::main` is found
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_const_forbidden/imported_main_const_forbidden.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args entry-point/imported_main_const_forbidden.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/entry-point/imported_main_const_forbidden.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_const_forbidden" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_const_forbidden/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0601]: `main` function not found in crate `imported_main_const_forbidden`
  --> /checkout/src/test/ui/entry-point/imported_main_const_forbidden.rs:7:22
   |
LL | use foo::BAR as main;
   |     ---------------- ^ consider adding a `main` function to `/checkout/src/test/ui/entry-point/imported_main_const_forbidden.rs`
   |     |
   |     non-function item at `crate::main` is found
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.

---
1 error[E0601]: `main` function not found in crate `imported_main_const_fn_item_type_forbidden`
-   --> $DIR/imported_main_const_fn_item_type_forbidden.rs:1:1
+   --> $DIR/imported_main_const_fn_item_type_forbidden.rs:13:22
3    |
- LL | / #![feature(imported_main)]
- LL | | #![feature(type_alias_impl_trait)]
- LL | | #![allow(incomplete_features)]
- LL | |
- LL | |
- LL | |
- LL | | use foo::BAR as main;
-    | |_____----------------^ consider adding a `main` function to `$DIR/imported_main_const_fn_item_type_forbidden.rs`
-    |       |
-    |       non-function item at `crate::main` is found
+ LL | use foo::BAR as main;
+    |     ---------------- ^ consider adding a `main` function to `$DIR/imported_main_const_fn_item_type_forbidden.rs`
+    |     |
+    |     non-function item at `crate::main` is found
15 error[E0308]: mismatched types
-   --> $DIR/imported_main_const_fn_item_type_forbidden.rs:10:29
+   --> $DIR/imported_main_const_fn_item_type_forbidden.rs:9:29
17    |
17    |
18 LL |     type MainFn = impl Fn();


25                   found fn item `fn() {bar}`
27 error: could not find defining uses
-   --> $DIR/imported_main_const_fn_item_type_forbidden.rs:6:19
+   --> $DIR/imported_main_const_fn_item_type_forbidden.rs:5:19
29    |
29    |
30 LL |     type MainFn = impl Fn();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_const_fn_item_type_forbidden/imported_main_const_fn_item_type_forbidden.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_const_fn_item_type_forbidden/imported_main_const_fn_item_type_forbidden.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args entry-point/imported_main_const_fn_item_type_forbidden.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/entry-point/imported_main_const_fn_item_type_forbidden.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_const_fn_item_type_forbidden" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_const_fn_item_type_forbidden/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0601]: `main` function not found in crate `imported_main_const_fn_item_type_forbidden`
  --> /checkout/src/test/ui/entry-point/imported_main_const_fn_item_type_forbidden.rs:13:22
   |
LL | use foo::BAR as main; //~ ERROR `main` function not found in crate
   |     ---------------- ^ consider adding a `main` function to `/checkout/src/test/ui/entry-point/imported_main_const_fn_item_type_forbidden.rs`
   |     |
   |     non-function item at `crate::main` is found
error[E0308]: mismatched types
  --> /checkout/src/test/ui/entry-point/imported_main_const_fn_item_type_forbidden.rs:9:29
   |
   |
LL |     type MainFn = impl Fn();
...
...
LL |     pub const BAR: MainFn = bar;
   |                             ^^^ expected opaque type, found fn item
   = note: expected opaque type `impl Fn()`
   = note: expected opaque type `impl Fn()`
                  found fn item `fn() {bar}`
error: could not find defining uses
  --> /checkout/src/test/ui/entry-point/imported_main_const_fn_item_type_forbidden.rs:5:19
   |
   |
LL |     type MainFn = impl Fn();

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0601.
---
1 error[E0601]: `main` function not found in crate `missing_main`
-   --> $DIR/missing-main.rs:2:1
+   --> $DIR/missing-main.rs:2:14
3    |
4 LL | fn mian() { }
-    | ^^^^^^^^^^^^^ consider adding a `main` function to `$DIR/missing-main.rs`
+    |              ^ consider adding a `main` function to `$DIR/missing-main.rs`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-main/missing-main.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args missing/missing-main.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-main" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0601]: `main` function not found in crate `missing_main`
  --> /checkout/src/test/ui/missing/missing-main.rs:2:14
   |
LL | fn mian() { }
   |              ^ consider adding a `main` function to `/checkout/src/test/ui/missing/missing-main.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.

---
To only update this specific test, also pass `--test-args parser/issues/issue-49040.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-49040.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-49040" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-49040/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected item, found `;`
  --> /checkout/src/test/ui/parser/issues/issue-49040.rs:1:28
   |
LL | #![allow(unused_variables)]; //~ ERROR expected item, found `;`

error[E0601]: `main` function not found in crate `issue_49040`
  --> /checkout/src/test/ui/parser/issues/issue-49040.rs:1:29
   |
   |
LL | #![allow(unused_variables)]; //~ ERROR expected item, found `;`
   |                             ^ consider adding a `main` function to `/checkout/src/test/ui/parser/issues/issue-49040.rs`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0601`.

---
test result: FAILED. 12420 passed; 4 failed; 116 ignored; 0 measured; 0 filtered out; finished in 148.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:18
