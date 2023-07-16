plain
..........................................i......................................................... 6300/12060
.................................................................................................... 6400/12060
ii.ii.......i...i................................................................................... 6500/12060
..............................................i....i..................................i............. 6600/12060
..............i.......................................................F..F.FF....................... 6700/12060
.................................................................................................... 6900/12060
..............................................................ii.................................... 7000/12060
.............i...................................................................................... 7100/12060
.................................................................................................... 7200/12060
---
diff of stderr:

61    |            ^^^^^^^^^^
62 
63 error: type alias impl traits are not allowed as field types in structs
-   --> $DIR/feature-gate-type_alias_impl_trait.rs:8:12
65    |
66 LL | type Foo = impl Debug;
67    | ---------------------- type alias defined here


68 ...
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
69 LL | struct Bar(Foo);
+    | ^^^^^^^^^^^---^^
+    |            |
+    |            this field contains a type alias impl trait
71 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-type_alias_impl_trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_alias_impl_trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-type_alias_impl_trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:11:9
   |
LL | type Foo = impl Debug;
   |            ---------- the expected opaque type
...
LL |     Bar(42) //~ ERROR mismatched types
   |         ^^ expected opaque type, found integer
   = note: expected opaque type `impl Debug`
   = note: expected opaque type `impl Debug`
                     found type `{integer}`
error[E0658]: type alias impl trait is not permitted here
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:17:19
   |
   |
LL |     let x = || -> Foo2 { 42 }; //~ ERROR not permitted here
   |
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
   = help: add `#![feature(type_alias_impl_trait)]` to the crate attributes to enable
error[E0308]: mismatched types
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:24:18
   |
LL | type Foo3 = impl Debug;
LL | type Foo3 = impl Debug;
   |             ---------- the found opaque type
...
LL |     let y: i32 = x; //~ ERROR mismatched types
   |            ---   ^ expected `i32`, found opaque type
   |            expected due to this
   |
   = note:     expected type `i32`
           found opaque type `impl Debug`
           found opaque type `impl Debug`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:27:13
   |
LL | type Foo3 = impl Debug;
   |             ---------- the expected opaque type
...
LL |     define3(42) //~ ERROR mismatched types
   |             ^^ expected opaque type, found integer
   = note: expected opaque type `impl Debug`
   = note: expected opaque type `impl Debug`
                     found type `{integer}`
error[E0658]: type alias impl trait is not permitted here
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:34:12
   |
LL |     let y: Foo4 = 42;
---
   |
LL | type Foo = impl Debug;
   |            ^^^^^^^^^^

error: type alias impl traits are not allowed as field types in structs
   |
LL | type Foo = impl Debug;
   | ---------------------- type alias defined here
...
...
LL | struct Bar(Foo);
   | ^^^^^^^^^^^---^^
   |            this field contains a type alias impl trait

error: could not find defining uses
  --> /checkout/src/test/ui/feature-gates/feature-gate-type_alias_impl_trait.rs:20:13
---
diff of stderr:

8    = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
9 
10 error: type alias impl traits are not allowed as field types in structs
-   --> $DIR/lint-ctypes-73249-3.rs:17:5
12    |
12    |
- LL | type Qux = impl Baz;
-    | -------------------- type alias defined here
+ LL |   type Qux = impl Baz;
+    |   -------------------- type alias defined here
15 ...
- LL |     x: Qux,
-    |     ^^^^^^
+ LL | / pub struct A {
+ LL | |     x: Qux,
+    | |     ------ this field contains a type alias impl trait
+ LL | |
+ LL | | }
18 
19 error: aborting due to previous error; 1 warning emitted
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.full_tait/lint-ctypes-73249-3.full_tait.stderr
To only update this specific test, also pass `--test-args lint/lint-ctypes-73249-3.rs`


error in revision `full_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.full_tait/auxiliary"
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


error: type alias impl traits are not allowed as field types in structs
   |
   |
LL |   type Qux = impl Baz;
   |   -------------------- type alias defined here
...
LL | / pub struct A {
LL | |     x: Qux,
   | |     ------ this field contains a type alias impl trait
LL | |     //~^ ERROR type alias impl traits are not allowed as field types in structs
LL | | }

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [ui] ui/lint/lint-ctypes-73249-3.rs#min_tait stdout ----
diff of stderr:

1 error: type alias impl traits are not allowed as field types in structs
-   --> $DIR/lint-ctypes-73249-3.rs:17:5
3    |
3    |
- LL | type Qux = impl Baz;
-    | -------------------- type alias defined here
+ LL |   type Qux = impl Baz;
+    |   -------------------- type alias defined here
6 ...
- LL |     x: Qux,
-    |     ^^^^^^
+ LL | / pub struct A {
+ LL | |     x: Qux,
+    | |     ------ this field contains a type alias impl trait
+ LL | |
+ LL | | }
9 
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.min_tait/lint-ctypes-73249-3.min_tait.stderr
To only update this specific test, also pass `--test-args lint/lint-ctypes-73249-3.rs`


error in revision `min_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-3.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type alias impl traits are not allowed as field types in structs
   |
   |
LL |   type Qux = impl Baz;
   |   -------------------- type alias defined here
...
LL | / pub struct A {
LL | |     x: Qux,
   | |     ------ this field contains a type alias impl trait
LL | |     //~^ ERROR type alias impl traits are not allowed as field types in structs
LL | | }

error: aborting due to previous error


---
diff of stderr:

8    = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
9 
10 error: type alias impl traits are not allowed as field types in structs
-   --> $DIR/lint-ctypes-73249-5.rs:17:5
12    |
12    |
- LL | type Qux = impl Baz;
-    | -------------------- type alias defined here
+ LL |   type Qux = impl Baz;
+    |   -------------------- type alias defined here
15 ...
- LL |     x: Qux,
-    |     ^^^^^^
+ LL | / pub struct A {
+ LL | |     x: Qux,
+    | |     ------ this field contains a type alias impl trait
+ LL | |
+ LL | | }
18 
19 error: aborting due to previous error; 1 warning emitted
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.full_tait/lint-ctypes-73249-5.full_tait.stderr
To only update this specific test, also pass `--test-args lint/lint-ctypes-73249-5.rs`


error in revision `full_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.full_tait/auxiliary"
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


error: type alias impl traits are not allowed as field types in structs
   |
   |
LL |   type Qux = impl Baz;
   |   -------------------- type alias defined here
...
LL | / pub struct A {
LL | |     x: Qux,
   | |     ------ this field contains a type alias impl trait
LL | |     //~^ ERROR type alias impl traits are not allowed as field types in structs
LL | | }

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [ui] ui/lint/lint-ctypes-73249-5.rs#min_tait stdout ----
diff of stderr:

1 error: type alias impl traits are not allowed as field types in structs
-   --> $DIR/lint-ctypes-73249-5.rs:17:5
3    |
3    |
- LL | type Qux = impl Baz;
-    | -------------------- type alias defined here
+ LL |   type Qux = impl Baz;
+    |   -------------------- type alias defined here
6 ...
- LL |     x: Qux,
-    |     ^^^^^^
+ LL | / pub struct A {
+ LL | |     x: Qux,
+    | |     ------ this field contains a type alias impl trait
+ LL | |
+ LL | | }
9 
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.min_tait/lint-ctypes-73249-5.min_tait.stderr
To only update this specific test, also pass `--test-args lint/lint-ctypes-73249-5.rs`


error in revision `min_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-73249-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-73249-5.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type alias impl traits are not allowed as field types in structs
   |
   |
LL |   type Qux = impl Baz;
   |   -------------------- type alias defined here
...
LL | / pub struct A {
LL | |     x: Qux,
   | |     ------ this field contains a type alias impl trait
LL | |     //~^ ERROR type alias impl traits are not allowed as field types in structs
LL | | }

error: aborting due to previous error


---
test result: FAILED. 11955 passed; 5 failed; 100 ignored; 0 measured; 0 filtered out; finished in 102.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:45
