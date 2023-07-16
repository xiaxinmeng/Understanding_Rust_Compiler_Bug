plain
.................................................................................................... 9400/12080
.................................................................................................... 9500/12080
.................................................................................................... 9600/12080
.................................................................................................... 9700/12080
.................F.............................................................................ii.ii 9800/12080
...........................................................................iiiiii.i..iiiiii.i....... 10000/12080
.................................................................................................... 10100/12080
.................................................................................................... 10200/12080
.................................................................................................... 10300/12080
---

---- [ui] ui/asm/naked-invalid-attr.rs stdout ----
diff of stderr:

38 LL | #![naked]
40 
- error: aborting due to 5 previous errors
- error: aborting due to 5 previous errors
+ warning: naked functions require #[inline(never)]
+   --> $DIR/naked-invalid-attr.rs:32:1
+    |
+ LL | #[naked]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    = note: `#[warn(unsupported_naked_functions)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
+ warning: naked functions require #[inline(never)]
+   --> $DIR/naked-invalid-attr.rs:26:5
+    |
+ LL |     #[naked]
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
+ 
+ warning: naked functions require #[inline(never)]
+   --> $DIR/naked-invalid-attr.rs:38:5
+    |
+ LL |     #[naked]
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
+ 
+ warning: naked functions require #[inline(never)]
+   --> $DIR/naked-invalid-attr.rs:43:5
+    |
+ LL |     #[naked]
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
---

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-invalid-attr/naked-invalid-attr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/naked-invalid-attr.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/naked-invalid-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-invalid-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/naked-invalid-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:13:1
   |
LL |   #[naked] //~ ERROR should be applied to a function definition
   |   ^^^^^^^^
LL |   #[repr(C)]
LL | / struct S {
LL | |     a: u32,
LL | |     b: u32,
LL | | }
   | |_- not a function definition
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:50:5
   |
   |
LL |     #[naked] || {}; //~ ERROR should be applied to a function definition

error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:21:5
   |
   |
LL |     #[naked] //~ ERROR should be applied to a function definition
LL |     extern "C" fn invoke(&self);
   |     ---------------------------- not a function definition

error: attribute should be applied to a function definition
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:9:5
   |
LL |     #[naked] //~ ERROR should be applied to a function definition
LL |     fn f();
   |     ------- not a function definition

error: attribute should be applied to a function definition
error: attribute should be applied to a function definition
  --> /checkout/src/test/ui/asm/naked-invalid-attr.rs:6:1
   |
LL | #![naked] //~ ERROR should be applied to a function definition


warning: naked functions require #[inline(never)]
   |
   |
LL | #[naked]
   |
   = note: `#[warn(unsupported_naked_functions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions require #[inline(never)]
   |
   |
LL |     #[naked]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions require #[inline(never)]
   |
   |
LL |     #[naked]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>


warning: naked functions require #[inline(never)]
   |
   |
LL |     #[naked]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---
16    = note: see issue #32408 <https://github.com/rust-lang/rust/issues/32408> for more information
17    = help: add `#![feature(naked_functions)]` to the crate attributes to enable
18 
- error: aborting due to 2 previous errors
+ warning: naked functions require #[inline(never)]
+    |
+    |
+ LL | #[naked]
+    |
+    = note: `#[warn(unsupported_naked_functions)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
+ warning: naked functions require #[inline(never)]
+    |
+    |
+ LL | #[naked]
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-naked_functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-naked_functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-naked_functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-naked_functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: the `#[naked]` attribute is an experimental feature
   |
   |
LL | #[naked]
   |
   = note: see issue #32408 <https://github.com/rust-lang/rust/issues/32408> for more information
   = help: add `#![feature(naked_functions)]` to the crate attributes to enable


error[E0658]: the `#[naked]` attribute is an experimental feature
   |
   |
LL | #[naked]
   |
   = note: see issue #32408 <https://github.com/rust-lang/rust/issues/32408> for more information
   = help: add `#![feature(naked_functions)]` to the crate attributes to enable


warning: naked functions require #[inline(never)]
   |
   |
LL | #[naked]
   |
   = note: `#[warn(unsupported_naked_functions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions require #[inline(never)]
   |
   |
LL | #[naked]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---

---- [ui] ui/rfc-2091-track-caller/error-with-naked.rs stdout ----
diff of stderr:

10 LL |     #[track_caller]
12 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ warning: naked functions require #[inline(never)]
+    |
+    |
+ LL | #[naked]
+    |
+    = note: `#[warn(unsupported_naked_functions)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
+ warning: naked functions require #[inline(never)]
+    |
+    |
+ LL |     #[naked]
+    |
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+    = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
+ 
---
To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-naked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-naked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-naked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0736]: cannot use `#[track_caller]` with `#[naked]`
   |
   |
LL | #[track_caller] //~ ERROR cannot use `#[track_caller]` with `#[naked]`


error[E0736]: cannot use `#[track_caller]` with `#[naked]`
   |
   |
LL |     #[track_caller] //~ ERROR cannot use `#[track_caller]` with `#[naked]`


warning: naked functions require #[inline(never)]
   |
   |
LL | #[naked]
   |
   = note: `#[warn(unsupported_naked_functions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

warning: naked functions require #[inline(never)]
   |
   |
LL |     #[naked]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #32408 <https://github.com/rust-lang/rust/issues/32408>

---
test result: FAILED. 11974 passed; 3 failed; 103 ignored; 0 measured; 0 filtered out; finished in 126.79s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:46
