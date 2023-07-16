plain
.........................................F.......................................................... 4300/12151
.................................................................................................... 4400/12151
.................................................................................................... 4500/12151
............ii...................................................................................... 4600/12151
................i...................................F..F............................................ 4700/12151
.................................................................................................... 4900/12151
.................................................................................................... 5000/12151
.................................................................................................... 5100/12151
.................................................................................................... 5200/12151
---
.................................................................................................... 11000/12151
.................................................................................................... 11100/12151
.................................................................................................... 11200/12151
.................................................................................................... 11300/12151
.........................F........................................................i.........F....... 11400/12151
.................................................................................................... 11600/12151
.................................................................................................... 11700/12151
.................................................................................................... 11800/12151
.................................................................................................... 11900/12151
---
---- [ui] ui/impl-trait/auto-trait-leak.rs stdout ----
diff of stderr:

34    |
35 LL |     send(cycle2().clone());
36    |     ^^^^
-    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
+    = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
38 note: ...which requires computing type of `cycle2::{opaque#0}`...
40    |

70    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
71 LL |     send(cycle1().clone());
72    |     ^^^^
-    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
+    = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
74    = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
75 note: cycle used when checking item types in top-level module


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
   |
   |
LL | fn cycle1() -> impl Clone {
   |
   |
note: ...which requires borrow-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle1`...
   |
   |
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle1`...
   |
   |
LL |     send(cycle2().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
note: ...which requires computing type of `cycle2::{opaque#0}`...
   |
   |
LL | fn cycle2() -> impl Clone {
   |                ^^^^^^^^^^
note: ...which requires borrow-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `cycle2`...
   |
   |
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `cycle2`...
   |
   |
LL |     send(cycle1().clone());
   |     ^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::clone::Clone: core::marker::Send`...
   = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
LL | / use std::cell::Cell;
LL | / use std::cell::Cell;
LL | | use std::rc::Rc;
LL | |
LL | | fn send<T: Send>(_: T) {}
...  |
LL | |     Rc::new(String::from("foo"))
LL | | }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
For more information about this error, try `rustc --explain E0391`.

------------------------------------------


---- [ui] ui/intrinsics/panic-uninitialized-zeroed.rs#mir stdout ----

error in revision `mir`: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/panic-uninitialized-zeroed.mir/a"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'attempted to instantiate uninhabited type `!`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:67:16
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'attempted to instantiate uninhabited type `!`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:71:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `!`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:75:43
thread 'main' panicked at 'attempted to instantiate uninhabited type `Foo`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:80:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Foo`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:84:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Foo`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:88:45
thread 'main' panicked at 'attempted to instantiate uninhabited type `Bar`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:93:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Bar`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:97:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Bar`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:101:45
thread 'main' panicked at 'attempted to leave type `fn()` uninitialized, which is invalid', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:107:16
thread 'main' panicked at 'attempted to zero-initialize type `fn()`, which is invalid', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:111:16
thread 'main' panicked at 'attempted to leave type `*const dyn core::marker::Send` uninitialized, which is invalid', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:116:16
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some("attempted to leave type `*const dyn core::marker::Send` uninitialized, which is invalid")`,
 right: `Some("attempted to leave type `*const dyn std::marker::Send` uninitialized, which is invalid")`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:57:5
------------------------------------------


---- [ui] ui/intrinsics/panic-uninitialized-zeroed.rs#thir stdout ----
---- [ui] ui/intrinsics/panic-uninitialized-zeroed.rs#thir stdout ----

error in revision `thir`: test run failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/panic-uninitialized-zeroed.thir/a"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'main' panicked at 'attempted to instantiate uninhabited type `!`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:67:16
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'attempted to instantiate uninhabited type `!`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:71:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `!`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:75:43
thread 'main' panicked at 'attempted to instantiate uninhabited type `Foo`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:80:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Foo`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:84:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Foo`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:88:45
thread 'main' panicked at 'attempted to instantiate uninhabited type `Bar`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:93:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Bar`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:97:16
thread 'main' panicked at 'attempted to instantiate uninhabited type `Bar`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:101:45
thread 'main' panicked at 'attempted to leave type `fn()` uninitialized, which is invalid', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:107:16
thread 'main' panicked at 'attempted to zero-initialize type `fn()`, which is invalid', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:111:16
thread 'main' panicked at 'attempted to leave type `*const dyn core::marker::Send` uninitialized, which is invalid', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:116:16
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `Some("attempted to leave type `*const dyn core::marker::Send` uninitialized, which is invalid")`,
 right: `Some("attempted to leave type `*const dyn std::marker::Send` uninitialized, which is invalid")`', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:57:5
------------------------------------------


---- [ui] ui/recursion/issue-26548-recursion-via-normalize.rs stdout ----
---- [ui] ui/recursion/issue-26548-recursion-via-normalize.rs stdout ----
diff of stderr:

1 error[E0391]: cycle detected when computing layout of `S`
2    |
-    = note: ...which requires computing layout of `std::option::Option<<S as Mirror>::It>`...
-    = note: ...which requires computing layout of `std::option::Option<S>`...
+    = note: ...which requires computing layout of `core::option::Option<<S as Mirror>::It>`...
+    = note: ...which requires computing layout of `core::option::Option<S>`...
5    = note: ...which again requires computing layout of `S`, completing the cycle
-    = note: cycle used when computing layout of `std::option::Option<S>`
+    = note: cycle used when computing layout of `core::option::Option<S>`
8 error: aborting due to previous error
9 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-26548-recursion-via-normalize/issue-26548-recursion-via-normalize.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/issue-26548-recursion-via-normalize.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-26548-recursion-via-normalize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-26548-recursion-via-normalize" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-26548-recursion-via-normalize/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing layout of `S`
   |
   = note: ...which requires computing layout of `core::option::Option<<S as Mirror>::It>`...
   = note: ...which requires computing layout of `core::option::Option<S>`...
   = note: ...which again requires computing layout of `S`, completing the cycle
   = note: cycle used when computing layout of `core::option::Option<S>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.


------------------------------------------


---- [ui] ui/type-alias-impl-trait/auto-trait-leakage3.rs stdout ----
diff of stderr:

9    |
10 LL |         is_send(foo());
11    |         ^^^^^^^
-    = note: ...which requires evaluating trait selection obligation `impl std::fmt::Debug: std::marker::Send`...
+    = note: ...which requires evaluating trait selection obligation `impl core::fmt::Debug: core::marker::Send`...
13    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
14 note: cycle used when checking item types in module `m`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3/auto-trait-leakage3.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3/auto-trait-leakage3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/auto-trait-leakage3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/auto-trait-leakage3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/auto-trait-leakage3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
   |
LL |         is_send(foo());
   |         ^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::fmt::Debug: core::marker::Send`...
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
---
---- [ui] ui/type-alias-impl-trait/inference-cycle.rs stdout ----
diff of stderr:

9    |
10 LL |         is_send(foo()); // Today: error
11    |         ^^^^^^^
-    = note: ...which requires evaluating trait selection obligation `impl std::fmt::Debug: std::marker::Send`...
+    = note: ...which requires evaluating trait selection obligation `impl core::fmt::Debug: core::marker::Send`...
13    = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
14 note: cycle used when checking item types in module `m`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle/inference-cycle.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle/inference-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/inference-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/inference-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/inference-cycle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing type of `m::Foo::{opaque#0}`
   |
LL |     type Foo = impl std::fmt::Debug;
   |                ^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires type-checking `m::bar`...
   |
   |
LL |         is_send(foo()); // Today: error
   |         ^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl core::fmt::Debug: core::marker::Send`...
   = note: ...which again requires computing type of `m::Foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in module `m`
   |
   |
LL | mod m {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-impl-trait/inference-cycle.rs:19:22
   |
   |
LL |     type Foo = impl std::fmt::Debug;
   |                -------------------- the expected opaque type
...
LL |         let f: Foo = 22_u32;
   |                ---   ^^^^^^ expected opaque type, found `u32`
   |                expected due to this
   |
   = note: expected opaque type `impl Debug`
                     found type `u32`
---
test result: FAILED. 12043 passed; 6 failed; 102 ignored; 0 measured; 0 filtered out; finished in 125.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:14
