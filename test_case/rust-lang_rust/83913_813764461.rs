plain

84 LL | | }
85    | |_^
86 
+ error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    |
+    |
+ note: ...which requires borrow-checking `cycle1`...
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing MIR for `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires unsafety-checking `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires building MIR for `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires type-checking `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
+ note: ...which requires computing type of `cycle2::{opaque#0}`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    |                ^^^^^^^^^^
+ note: ...which requires borrow-checking `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing MIR for `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires unsafety-checking `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires building MIR for `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires type-checking `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in top-level module
+    |
+    |
+ LL | / use std::cell::Cell;
+ LL | | use std::rc::Rc;
+ LL | |
+ LL | | fn send<T: Send>(_: T) {}
+ ...  |
+ LL | |     Rc::new(String::from("foo"))
+ LL | | }
+ 
+ 
+ error[E0391]: cycle detected when computing type of `cycle1::{opaque#0}`
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    |
+    |
+ note: ...which requires borrow-checking `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing MIR for `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires unsafety-checking `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires building MIR for `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires type-checking `cycle1`...
+    |
+    |
+ LL | fn cycle1() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
+ note: ...which requires computing type of `cycle2::{opaque#0}`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    |                ^^^^^^^^^^
+ note: ...which requires borrow-checking `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires processing MIR for `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires unsafety-checking `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires building MIR for `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+ note: ...which requires type-checking `cycle2`...
+    |
+    |
+ LL | fn cycle2() -> impl Clone {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: ...which again requires computing type of `cycle1::{opaque#0}`, completing the cycle
+ note: cycle used when checking item types in top-level module
+    |
+    |
+ LL | / use std::cell::Cell;
+ LL | | use std::rc::Rc;
+ LL | |
+ LL | | fn send<T: Send>(_: T) {}
+ ...  |
+ LL | |     Rc::new(String::from("foo"))
+ LL | | }
+ 
+ 
87 error[E0277]: `Rc<String>` cannot be sent between threads safely
89    |


99    = help: within `impl Clone`, the trait `Send` is not implemented for `Rc<String>`
100    = note: required because it appears within the type `impl Clone`
- error: aborting due to 2 previous errors
+ error: aborting due to 4 previous errors
103 
104 Some errors have detailed explanations: E0277, E0391.
104 Some errors have detailed explanations: E0277, E0391.
105 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary"
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
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
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
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
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
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
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
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
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
LL | fn cycle1() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
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
LL | fn cycle2() -> impl Clone {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
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


error[E0277]: `Rc<String>` cannot be sent between threads safely
   |
   |
LL | fn send<T: Send>(_: T) {}
   |            ---- required by this bound in `send`
...
LL |     send(cycle2().clone());
   |     ^^^^ `Rc<String>` cannot be sent between threads safely
...
LL | fn cycle2() -> impl Clone {
   |                ---------- within this `impl Clone`
   |
   = help: within `impl Clone`, the trait `Send` is not implemented for `Rc<String>`
   = note: required because it appears within the type `impl Clone`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0391.
For more information about an error, try `rustc --explain E0277`.
---
test result: FAILED. 11636 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 138.49s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:09
