plain
.................................................................................................... 3500/11450
.................................................................................................... 3600/11450
.......................i............................................................................ 3700/11450
.................................................................................................... 3800/11450
..........................................F.....F................................................... 3900/11450
.................................................................................................... 4100/11450
.................................................................................................... 4200/11450
.................................................................................................... 4300/11450
...........................ii....................................................................... 4400/11450
---
.................................................................................................... 9200/11450
.................................................................................................... 9300/11450
.................................................................................................... 9400/11450
.........i......i................................................................................... 9500/11450
...............................................iiiiiii..iiiiii.i.................................... 9600/11450
.................................................................................................... 9800/11450
.................................................................................................... 9900/11450
.................................................................................................... 10000/11450
.................................................................................................... 10100/11450
---

---- [ui] ui/generic-associated-types/issue-76535.rs stdout ----
diff of stderr:

23 LL |     type SubType<'a><'a>: SubTrait;
25 
25 
- error[E0038]: the trait `SuperTrait` cannot be made into an object
-   --> $DIR/issue-76535.rs:38:14
-    |
- LL |     let sub: Box<dyn SuperTrait<SubType = SubStruct>> = Box::new(SuperStruct::new(0));
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SuperTrait` cannot be made into an object
-    |
-    = help: consider moving `get_sub` to another trait
- note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/issue-76535.rs:10:37
-    |
- LL | pub trait SuperTrait {
-    |           ---------- this trait cannot be made into an object...
- ...
- LL |     fn get_sub<'a>(&'a mut self) -> Self::SubType<'a>;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                                     ^^^^^^^^^^^^^^^^^ ...because method `get_sub` references the `Self` type in its return type
+ error: aborting due to previous error; 1 warning emitted
41 
- error[E0038]: the trait `SuperTrait` cannot be made into an object
-   --> $DIR/issue-76535.rs:38:57
-    |
- LL |     let sub: Box<dyn SuperTrait<SubType = SubStruct>> = Box::new(SuperStruct::new(0));
-    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SuperTrait` cannot be made into an object
-    |
-    = help: consider moving `get_sub` to another trait
- note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/issue-76535.rs:10:37
-    |
- LL | pub trait SuperTrait {
-    |           ---------- this trait cannot be made into an object...
- ...
- LL |     fn get_sub<'a>(&'a mut self) -> Self::SubType<'a>;
-    |                                     ^^^^^^^^^^^^^^^^^ ...because method `get_sub` references the `Self` type in its return type
-    = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn SuperTrait<SubType = SubStruct<'_>>>>` for `Box<SuperStruct>`
-    = note: required by cast to type `Box<dyn SuperTrait<SubType = SubStruct<'_>>>`
- error: aborting due to 3 previous errors; 1 warning emitted
- 
- Some errors have detailed explanations: E0038, E0107.
- For more information about an error, try `rustc --explain E0038`.
- For more information about an error, try `rustc --explain E0038`.
+ For more information about this error, try `rustc --explain E0107`.
64 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-76535/issue-76535.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-76535.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-76535.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-76535" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-76535/auxiliary"
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

error[E0107]: missing generics for associated type `SuperTrait::SubType`
  --> /checkout/src/test/ui/generic-associated-types/issue-76535.rs:7:10
   |
LL |     type SubType<'a>: SubTrait;
   |          ^^^^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type SubType<'a>: SubTrait;
   |          ^^^^^^^ --
help: use angle brackets to add missing lifetime argument
   |
LL |     type SubType<'a><'a>: SubTrait;

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0107`.
For more information about this error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-79422.rs stdout ----
diff of stderr:

14 LL |     type VRefCont<'a><'a>: RefCont<'a, V>;
16 
16 
- error[E0038]: the trait `MapLike` cannot be made into an object
-   --> $DIR/issue-79422.rs:44:12
-    |
- LL |         as Box<dyn MapLike<u8, u8, VRefCont = dyn RefCont<'_, u8>>>;
-    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
-    |
-    = help: consider moving `get` to another trait
- note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/issue-79422.rs:23:38
-    |
- LL | trait MapLike<K, V> {
-    |       ------- this trait cannot be made into an object...
- ...
- LL |     fn get<'a>(&'a self, key: &K) -> Option<Self::VRefCont<'a>>;
-    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...because method `get` references the `Self` type in its return type
- 
- error[E0038]: the trait `MapLike` cannot be made into an object
+ error[E0271]: type mismatch resolving `<BTreeMap<u8, u8> as MapLike<u8, u8>>::VRefCont<'static> == (dyn RefCont<'_, u8> + 'static)`
35    |
35    |
36 LL |     let m = Box::new(std::collections::BTreeMap::<u8, u8>::new())

-    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MapLike` cannot be made into an object
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn RefCont`, found reference
38    |
-    = help: consider moving `get` to another trait
- note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/issue-79422.rs:23:38
-    |
- LL | trait MapLike<K, V> {
-    |       ------- this trait cannot be made into an object...
- ...
- LL |     fn get<'a>(&'a self, key: &K) -> Option<Self::VRefCont<'a>>;
-    |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ ...because method `get` references the `Self` type in its return type
-    = note: required because of the requirements on the impl of `CoerceUnsized<Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>>` for `Box<BTreeMap<u8, u8>>`
-    = note: required by cast to type `Box<dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>>`
+    = note: expected trait object `(dyn RefCont<'_, u8> + 'static)`
+                  found reference `&'static u8`
+    = note: required for the cast to the object type `dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>`
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
52 
- Some errors have detailed explanations: E0038, E0107.
---


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


error[E0271]: type mismatch resolving `<BTreeMap<u8, u8> as MapLike<u8, u8>>::VRefCont<'static> == (dyn RefCont<'_, u8> + 'static)`
   |
   |
LL |     let m = Box::new(std::collections::BTreeMap::<u8, u8>::new())
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait object `dyn RefCont`, found reference
   |
   = note: expected trait object `(dyn RefCont<'_, u8> + 'static)`
                 found reference `&'static u8`
   = note: required for the cast to the object type `dyn MapLike<u8, u8, VRefCont = (dyn RefCont<'_, u8> + 'static)>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0107, E0271.
For more information about an error, try `rustc --explain E0107`.
---
test result: FAILED. 11355 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 110.76s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:08
