plain
.................................................................................................... 3500/11392
.................................................................................................... 3600/11392
..........i......................................................................................... 3700/11392
.................................................................................................... 3800/11392
........................F.....FF.................................................................... 3900/11392
.................................................................................................... 4100/11392
.................................................................................................... 4200/11392
.................................................................................................... 4300/11392
...........ii..................................................................................i.... 4400/11392
---
..................................................................................i.i............... 11300/11392
............................................................................................
failures:

---- [ui] ui/generic-associated-types/issue-80433-reduced.rs stdout ----
normalized stderr:
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
  --> $DIR/issue-80433-reduced.rs:3:12
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433-reduced/issue-80433-reduced.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-80433-reduced.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-80433-reduced.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433-reduced" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433-reduced/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/generic-associated-types/issue-80433-reduced.rs:3:12
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(incomplete_features)]` on by default
---

---- [ui] ui/generic-associated-types/issue-79422.rs stdout ----
diff of stderr:

+ error[E0658]: generic associated types are unstable
+    |
+    |
+ LL |     type VRefCont<'a>: RefCont<'a, V>;
+    |
+    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
+    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: generic associated types are unstable
+    |
+    |
+ LL |     type VRefCont<'a> = &'a V;
+    |
+    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
+    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
+ 
+ 
+ error[E0658]: generic associated types are unstable
+    |
+    |
+ LL |     type VRefCont<'a> = Box<V>;
+    |
+    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
+    = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable
+ 
+ 
1 error[E0107]: missing generics for associated type `MapLike::VRefCont`
-   --> $DIR/issue-79422.rs:21:10
3    |
3    |
4 LL |     type VRefCont<'a>: RefCont<'a, V>;
5    |          ^^^^^^^^ expected 1 lifetime argument
6    |
6    |
7 note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-79422.rs:21:10
9    |
9    |
10 LL |     type VRefCont<'a>: RefCont<'a, V>;

15    |                  ^^^^
16 
16 
17 error[E0038]: the trait `MapLike` cannot be made into an object
-   --> $DIR/issue-79422.rs:44:12
19    |
19    |
20 LL |         as Box<dyn MapLike<u8, u8, VRefCont = dyn RefCont<'_, u8>>>;
21    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
22    |
22    |
23    = help: consider moving `get` to another trait
24 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/issue-79422.rs:23:38
26    |
26    |
27 LL | trait MapLike<K, V> {
28    |       ------- this trait cannot be made into an object...

31    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...because method `get` references the `Self` type in its return type
32 
33 error[E0038]: the trait `MapLike` cannot be made into an object
-   --> $DIR/issue-79422.rs:43:13
35    |
35    |
36 LL |     let m = Box::new(std::collections::BTreeMap::<u8, u8>::new())
37    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
38    |
38    |
39    = help: consider moving `get` to another trait
40 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/issue-79422.rs:23:38
42    |
42    |
43 LL | trait MapLike<K, V> {
44    |       ------- this trait cannot be made into an object...

48    = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>>` for `Box<BTreeMap<u8, u8>>`
49    = note: required by cast to type `Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>`
- error: aborting due to 3 previous errors
+ error: aborting due to 6 previous errors
52 
- Some errors have detailed explanations: E0038, E0107.
- Some errors have detailed explanations: E0038, E0107.
+ Some errors have detailed explanations: E0038, E0107, E0658.
54 For more information about an error, try `rustc --explain E0038`.
55 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-79422/issue-79422.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-79422.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-79422.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-79422" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-79422/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: generic associated types are unstable
   |
   |
LL |     type VRefCont<'a>: RefCont<'a, V>;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: generic associated types are unstable
   |
   |
LL |     type VRefCont<'a> = &'a V;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0658]: generic associated types are unstable
   |
   |
LL |     type VRefCont<'a> = Box<V>;
   |
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
   = help: add `#![feature(generic_associated_types)]` to the crate attributes to enable


error[E0107]: missing generics for associated type `MapLike::VRefCont`
   |
   |
LL |     type VRefCont<'a>: RefCont<'a, V>;
   |          ^^^^^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type VRefCont<'a>: RefCont<'a, V>;
   |          ^^^^^^^^ --
help: use angle brackets to add missing lifetime argument
   |
LL |     type VRefCont<'a><'a>: RefCont<'a, V>;


error[E0038]: the trait `MapLike` cannot be made into an object
   |
   |
LL |         as Box<dyn MapLike<u8, u8, VRefCont = dyn RefCont<'_, u8>>>;
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
   |
   = help: consider moving `get` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL | trait MapLike<K, V> {
   |       ------- this trait cannot be made into an object...
...
LL |     fn get<'a>(&'a self, key: &K) -> Option<Self::VRefCont<'a>>;
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...because method `get` references the `Self` type in its return type

error[E0038]: the trait `MapLike` cannot be made into an object
   |
   |
LL |     let m = Box::new(std::collections::BTreeMap::<u8, u8>::new())
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
   |
   = help: consider moving `get` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL | trait MapLike<K, V> {
   |       ------- this trait cannot be made into an object...
...
LL |     fn get<'a>(&'a self, key: &K) -> Option<Self::VRefCont<'a>>;
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...because method `get` references the `Self` type in its return type
   = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>>` for `Box<BTreeMap<u8, u8>>`
   = note: required by cast to type `Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0038, E0107, E0658.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-80433.rs stdout ----
diff of stderr:

+ warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
+    |
+    |
+ LL | #![feature(generic_associated_types)]
+    |
+    = note: `#[warn(incomplete_features)]` on by default
+    = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information
+ 
+ 
1 error[E0107]: missing generics for associated type `TestMut::Output`
-   --> $DIR/issue-80433.rs:10:10
+   --> $DIR/issue-80433.rs:9:10
3    |
4 LL |     type Output<'a>;
5    |          ^^^^^^ expected 1 lifetime argument
6    |
6    |
7 note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-80433.rs:10:10
9    |
9    |
10 LL |     type Output<'a>;


14 LL |     type Output<'a><'a>;
16 
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
18 
18 
19 For more information about this error, try `rustc --explain E0107`.
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433/issue-80433.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-80433.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-80433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-80433/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

error[E0107]: missing generics for associated type `TestMut::Output`
  --> /checkout/src/test/ui/generic-associated-types/issue-80433.rs:9:10
   |
LL |     type Output<'a>;
   |          ^^^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Output<'a>;
   |          ^^^^^^ --
help: use angle brackets to add missing lifetime argument
   |
LL |     type Output<'a><'a>;

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0107`.
---
test result: FAILED. 11297 passed; 3 failed; 92 ignored; 0 measured; 0 filtered out; finished in 141.90s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:06
