plain
.................................................ii................................................. 800/12259
.................................................................................................... 900/12259
.................................................................................................... 1000/12259
.................................................................................................... 1100/12259
.....................................................................i...........F.F................ 1200/12259
.................................................................................................... 1400/12259
.................................................................................................... 1500/12259
............................i.....i................i................................................ 1600/12259
.................................................................................................... 1700/12259
---
...............................................i.i.................................................. 12200/12259
...........................................................
failures:

---- [ui] ui/borrowck/issue-88434-minimal-example.rs stdout ----

+ warning: the feature `const_panic` has been stable since 1.57.0 and no longer requires an attribute to enable
+   --> $DIR/issue-88434-minimal-example.rs:2:12
+    |
---
18 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example/issue-88434-minimal-example.stderr
To only update this specific test, also pass `--test-args borrowck/issue-88434-minimal-example.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-minimal-example/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |
LL | const _CONST: &() = &f(&|_| {});
   |                      ---------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:5:22
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:11:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-minimal-example.rs:5:25: 5:31]>` at /checkout/library/std/src/panic.rs:18:9
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error; 1 warning emitted


For more information about this error, try `rustc --explain E0080`.

------------------------------------------


---- [ui] ui/borrowck/issue-88434-removal-index-should-be-less.rs stdout ----

+ warning: the feature `const_panic` has been stable since 1.57.0 and no longer requires an attribute to enable
+ warning: the feature `const_panic` has been stable since 1.57.0 and no longer requires an attribute to enable
+   --> $DIR/issue-88434-removal-index-should-be-less.rs:2:12
+ LL | #![feature(const_panic)]
+    |            ^^^^^^^^^^^
+    |
+    = note: `#[warn(stable_features)]` on by default
+    = note: `#[warn(stable_features)]` on by default
+ 
1 error[E0080]: evaluation of constant value failed
2   --> $DIR/issue-88434-removal-index-should-be-less.rs:11:5

12    |
13    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
14 
---
18 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less/issue-88434-removal-index-should-be-less.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-88434-removal-index-should-be-less.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-88434-removal-index-should-be-less/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `const_panic` has been stable since 1.57.0 and no longer requires an attribute to enable
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:2:12
LL | #![feature(const_panic)]
   |            ^^^^^^^^^^^
   |
   = note: `#[warn(stable_features)]` on by default
   = note: `#[warn(stable_features)]` on by default

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:11:5
   |
LL | const _CONST: &[u8] = &f(&[], |_| {});
   |                        -------------- inside `_CONST` at /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:5:24
...
LL |     panic!() //~ ERROR evaluation of constant value failed
   |     |
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:11:5
   |     inside `f::<[closure@/checkout/src/test/ui/borrowck/issue-88434-removal-index-should-be-less.rs:5:31: 5:37]>` at /checkout/library/std/src/panic.rs:18:9
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error; 1 warning emitted


For more information about this error, try `rustc --explain E0080`.

------------------------------------------



failures:
    [ui] ui/borrowck/issue-88434-minimal-example.rs
    [ui] ui/borrowck/issue-88434-removal-index-should-be-less.rs
test result: FAILED. 12142 passed; 2 failed; 115 ignored; 0 measured; 0 filtered out; finished in 128.35s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:20
