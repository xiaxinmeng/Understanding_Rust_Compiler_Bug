plain
................................................................................F................... 9000/11243
.................................................................................................... 9100/11243
.................................................................................................... 9200/11243
.......................................i......i..................................................... 9300/11243
.............................................................F................iiiiii..iiiiii.i...... 9400/11243
.................................................................................................... 9600/11243
.................................................................................................... 9700/11243
.................................................................................................... 9800/11243
.................................................................................................... 9900/11243
---
---- [ui] ui/imports/extern-crate-used.rs stdout ----
diff of stderr:

- error: unused extern crate
+ error: `extern crate` is not idiomatic in the new edition
3    |
4 LL | extern crate core;

-    | ^^^^^^^^^^^^^^^^^^ help: remove it
-    | ^^^^^^^^^^^^^^^^^^ help: remove it
+    | ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
6    |
7 note: the lint level is defined here
8   --> $DIR/extern-crate-used.rs:6:9


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-used/extern-crate-used.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/extern-crate-used.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/extern-crate-used.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-used" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-crate-used/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `extern crate` is not idiomatic in the new edition
   |
   |
LL | extern crate core; //~ ERROR unused extern crate
   | ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
note: the lint level is defined here
  --> /checkout/src/test/ui/imports/extern-crate-used.rs:6:9
   |
   |
LL | #![deny(unused_extern_crates)]

error: aborting due to previous error


---
11    |         ^^^^^^^^^^^^^^^^
12    = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`
13 
- warning: unused extern crate
+ warning: `extern crate` is not idiomatic in the new edition
16    |
17 LL | extern crate core;

-    | ^^^^^^^^^^^^^^^^^^ help: remove it
---
23 LL |     extern crate removing_extern_crate as foo;
24    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove it
25 
- warning: unused extern crate
+ warning: `extern crate` is not idiomatic in the new edition
28    |
29 LL |     extern crate core;

-    |     ^^^^^^^^^^^^^^^^^^ help: remove it
---
diff of fixed:

6 #![warn(rust_2018_idioms)]
7 
8  //~ WARNING unused extern crate
-  //~ WARNING unused extern crate
+ use core; //~ WARNING unused extern crate
11 mod another {
11 mod another {
12      //~ WARNING unused extern crate

-      //~ WARNING unused extern crate
+     use core; //~ WARNING unused extern crate
15 
16 fn main() {}



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate/removing-extern-crate.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args removing-extern-crate.rs`
error: 2 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/removing-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/removing-extern-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused extern crate
  --> /checkout/src/test/ui/removing-extern-crate.rs:8:1
   |
LL | extern crate removing_extern_crate as foo; //~ WARNING unused extern crate
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/removing-extern-crate.rs:6:9
   |
   |
LL | #![warn(rust_2018_idioms)]
   |         ^^^^^^^^^^^^^^^^
   = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`

warning: `extern crate` is not idiomatic in the new edition
   |
   |
LL | extern crate core; //~ WARNING unused extern crate
   | ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
warning: unused extern crate
  --> /checkout/src/test/ui/removing-extern-crate.rs:12:5
   |
   |
LL |     extern crate removing_extern_crate as foo; //~ WARNING unused extern crate


warning: `extern crate` is not idiomatic in the new edition
   |
   |
LL |     extern crate core; //~ WARNING unused extern crate
   |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
warning: 4 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/rust-2018/remove-extern-crate.rs stdout ----
diff of stderr:

- warning: unused extern crate
+ warning: `extern crate` is not idiomatic in the new edition
3    |
4 LL | extern crate core;

-    | ^^^^^^^^^^^^^^^^^^ help: remove it
---

6 
7 #![warn(rust_2018_idioms)]
8 
-  //~ WARNING unused extern crate
+ use core; //~ WARNING unused extern crate
10 // Shouldn't suggest changing to `use`, as `another_name`
11 // would no longer be added to the prelude which could cause
12 // compilation errors for imports that use `another_name` in other

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/remove-extern-crate.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/remove-extern-crate.rs`
error: 2 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/remove-extern-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "remove_extern_crate" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/remove-extern-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `extern crate` is not idiomatic in the new edition
   |
   |
LL | extern crate core; //~ WARNING unused extern crate
   | ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
note: the lint level is defined here
  --> /checkout/src/test/ui/rust-2018/remove-extern-crate.rs:7:9
   |
LL | #![warn(rust_2018_idioms)]
LL | #![warn(rust_2018_idioms)]
   |         ^^^^^^^^^^^^^^^^
   = note: `#[warn(unused_extern_crates)]` implied by `#[warn(rust_2018_idioms)]`

warning: `extern crate` is not idiomatic in the new edition
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     extern crate core; //~ WARNING `extern crate` is not idiomatic
   |     ^^^^^^^^^^^^^^^^^^ help: convert it to a `use`
warning: 2 warnings emitted


------------------------------------------
---
test result: FAILED. 11155 passed; 3 failed; 85 ignored; 0 measured; 0 filtered out; finished in 140.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:37
