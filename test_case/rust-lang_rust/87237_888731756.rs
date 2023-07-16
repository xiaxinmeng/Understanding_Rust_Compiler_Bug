plain
.................................................................................................... 2000/12070
..........................................i......................................................... 2100/12070
.................................................................................................... 2200/12070
.................................................................................................... 2300/12070
.......................................................................................F............ 2400/12070
..F..............................................................F.................................. 2500/12070
...........................................FF....................................................... 2600/12070
.........................................................................i..i....................... 2800/12070
.................................................................................................... 2900/12070
......................................................................................iiiii......... 3000/12070
.................................................................................................... 3100/12070
---
.................................................................................................... 5700/12070
.................................................................................................... 5800/12070
.................................................................................................... 5900/12070
.............................................................................i...................... 6000/12070
...............................F.F.................................................................. 6100/12070
...................................................i................................................ 6300/12070
.................................................................................................... 6400/12070
.............ii.ii.......i...i...................................................................... 6500/12070
............................................................i.....i................................. 6600/12070
---

---- [ui] ui/consts/const-fn-error.rs stdout ----
diff of stderr:

- error[E0744]: `for` is not allowed in a `const fn`
+ error[E0658]: `for` is not allowed in a `const fn`
3    |
4 LL | /     for i in 0..x {


9 LL | |         sum += i;
10 LL | |     }
+    |
+    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
12 
13 error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants

45 
46 error: aborting due to 5 previous errors
47 
---
To only update this specific test, also pass `--test-args consts/const-fn-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-fn-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `for` is not allowed in a `const fn`
   |
LL | /     for i in 0..x {
LL | /     for i in 0..x {
LL | |         //~^ ERROR mutable references
LL | |         //~| ERROR calls in constant functions
LL | |         //~| ERROR calls in constant functions
...  |
LL | |         sum += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
LL |     for i in 0..x {
   |              ^^^^

---
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
LL |     for i in 0..x {
   |              ^^^^

---
   |              |
   |              calling non-const function `<std::ops::Range<usize> as IntoIterator>::into_iter`
   |              inside `f` at /checkout/src/test/ui/consts/const-fn-error.rs:5:14
...
LL |     let a : [i32; f(X)];
   |                   ---- inside `main::{constant#0}` at /checkout/src/test/ui/consts/const-fn-error.rs:18:19
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0015, E0080, E0658.
For more information about an error, try `rustc --explain E0015`.
---
To only update this specific test, also pass `--test-args consts/const-for-feature-gate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-for-feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for-feature-gate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-for-feature-gate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
LL |     for _ in 0..5 {}
   |     ^^^^^^^^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args consts/const-try-feature-gate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-try-feature-gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-try-feature-gate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-try-feature-gate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `?` is not allowed in a `const fn`
   |
LL |     Some(())?;
   |     ^^^^^^^^^
   |
---

---- [ui] ui/consts/control-flow/loop.rs stdout ----
diff of stderr:

- error[E0744]: `for` is not allowed in a `const`
+ error[E0658]: `for` is not allowed in a `const`
3    |
4 LL | /     for i in 0..4 {


5 LL | |         x += i;
6 LL | |     }
+    |
+    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
8 
8 
- error[E0744]: `for` is not allowed in a `const`
+ error[E0658]: `for` is not allowed in a `const`
11    |
12 LL | /     for i in 0..4 {


13 LL | |         x += i;
14 LL | |     }
+    |
+    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
16 
---
To only update this specific test, also pass `--test-args consts/control-flow/loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/loop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/loop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
   |
LL | /     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
LL | |         x += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0658]: `for` is not allowed in a `const`
   |
   |
LL | /     for i in 0..4 { //~ ERROR `for` is not allowed in a `const`
LL | |         x += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable

---

---- [ui] ui/consts/control-flow/try.rs stdout ----
diff of stderr:

- error[E0744]: `?` is not allowed in a `const fn`
+ error[E0658]: `?` is not allowed in a `const fn`
2   --> $DIR/try.rs:6:5
4 LL |     x?;

5    |     ^^
+    |
---
To only update this specific test, also pass `--test-args consts/control-flow/try.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/try.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/try" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/try/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `?` is not allowed in a `const fn`
   |
   |
LL |     x?; //~ ERROR `?` is not allowed in a `const fn`
   |
   = note: see issue #87576 <https://github.com/rust-lang/rust/issues/87576> for more information
   = help: add `#![feature(const_try)]` to the crate attributes to enable

---

---- [ui] ui/issues/issue-50582.rs stdout ----
diff of stderr:

- error[E0744]: `for` is not allowed in a `const`
+ error[E0658]: `for` is not allowed in a `const`
3    |
3    |
4 LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
5    |                    ^^^^^^^^^^^^^^^^
+    |
+    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
6 
7 error[E0277]: cannot add `()` to `{integer}`

14 
15 error: aborting due to 2 previous errors
16 
---
To only update this specific test, also pass `--test-args issues/issue-50582.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50582.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50582" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50582/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
   |
LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0277]: cannot add `()` to `{integer}`
   |
   |
LL |     Vec::<[(); 1 + for x in 0..1 {}]>::new();
   |                  ^ no implementation for `{integer} + ()`
   |
   = help: the trait `Add<()>` is not implemented for `{integer}`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0658.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/issues/issue-50585.rs stdout ----
diff of stderr:

- error[E0744]: `for` is not allowed in a `const`
+ error[E0658]: `for` is not allowed in a `const`
3    |
3    |
4 LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
5    |                  ^^^^^^^^^^^^^^^^
+    |
+    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
---
To only update this specific test, also pass `--test-args issues/issue-50585.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50585.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50585" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50585/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `for` is not allowed in a `const`
   |
   |
LL |     |y: Vec<[(); for x in 0..2 {}]>| {};
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-50585.rs:2:18
   |
LL |     |y: Vec<[(); for x in 0..2 {}]>| {};

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0658.
---
---- [ui] ui/never_type/issue-52443.rs stdout ----
diff of stderr:

6    |
7    = note: `#[warn(while_true)]` on by default
8 
- error[E0744]: `for` is not allowed in a `const`
+ error[E0658]: `for` is not allowed in a `const`
11    |
11    |
12 LL |     [(); { for _ in 0usize.. {}; 0}];
13    |            ^^^^^^^^^^^^^^^^^^^^
+    |
+    = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
+    = help: add `#![feature(const_for)]` to the crate attributes to enable
---
To only update this specific test, also pass `--test-args never_type/issue-52443.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/issue-52443.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/issue-52443/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |     [(); {while true {break}; 0}];
   |           ^^^^^^^^^^ help: use `loop`
   |
   = note: `#[warn(while_true)]` on by default

error[E0658]: `for` is not allowed in a `const`
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0308]: mismatched types
  --> /checkout/src/test/ui/never_type/issue-52443.rs:2:10
   |
LL |     [(); & { loop { continue } } ]; //~ ERROR mismatched types
   |          |
   |          expected `usize`, found reference
   |          expected `usize`, found reference
   |          help: consider removing the borrow: `{ loop { continue } }`
   = note:   expected type `usize`
           found reference `&_`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/never_type/issue-52443.rs:4:17
   |
LL |     [(); loop { break }]; //~ ERROR mismatched types
   |                 |
   |                 expected `usize`, found `()`
   |                 expected `usize`, found `()`
   |                 help: give it a value of the expected type: `break 42`
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/never_type/issue-52443.rs:9:21
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];

error[E0658]: mutable references are not allowed in constants
  --> /checkout/src/test/ui/never_type/issue-52443.rs:9:21
   |
   |
LL |     [(); { for _ in 0usize.. {}; 0}];
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
  --> /checkout/src/test/ui/never_type/issue-52443.rs:9:21
   |
LL |     [(); { for _ in 0usize.. {}; 0}];

error: aborting due to 6 previous errors; 1 warning emitted

Some errors have detailed explanations: E0015, E0308, E0658.
---

---- [ui] ui/rfc-2632-const-trait-impl/hir-const-check.rs stdout ----
diff of stderr:

- error[E0744]: `?` is not allowed in a `const fn`
+ error[E0658]: `?` is not allowed in a `const fn`
2   --> $DIR/hir-const-check.rs:11:9
4 LL |         Some(())?;

5    |         ^^^^^^^^^
+    |
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/hir-const-check.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/hir-const-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/hir-const-check" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/hir-const-check/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `?` is not allowed in a `const fn`
   |
   |
LL |         Some(())?; //~ ERROR `?` is not allowed in a `const fn`
   |
   = note: see issue #87576 <https://github.com/rust-lang/rust/issues/87576> for more information
   = help: add `#![feature(const_try)]` to the crate attributes to enable

---
test result: FAILED. 11959 passed; 9 failed; 102 ignored; 0 measured; 0 filtered out; finished in 126.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:51
