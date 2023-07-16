plain
.................................................................................................... 9000/11245
.................................................................................................... 9100/11245
.................................................................................................... 9200/11245
.........................................i......i................................................... 9300/11245
................................................................................iiiiii..iiiiii.i.... 9400/11245
.................................................................................................... 9600/11245
.................................................................................................... 9700/11245
.................................................................................................... 9800/11245
.................................................................................................... 9900/11245
---
11 error[E0658]: panicking in constants is unstable
-   --> $DIR/feature-gate-const_panic.rs:9:15
+   --> $DIR/feature-gate-const_panic.rs:6:15
13    |
- LL | const X: () = unimplemented!();
-    |               ^^^^^^^^^^^^^^^^
+ LL | const Y: () = unreachable!();
16    |
17    = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
18    = help: add `#![feature(const_panic)]` to the crate attributes to enable


19    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
20 
21 error[E0658]: panicking in constants is unstable
-   --> $DIR/feature-gate-const_panic.rs:6:15
+   --> $DIR/feature-gate-const_panic.rs:9:15
23    |
- LL | const Y: () = unreachable!();
-    |               ^^^^^^^^^^^^^^
+ LL | const X: () = unimplemented!();
26    |
27    = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
28    = help: add `#![feature(const_panic)]` to the crate attributes to enable



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/feature-gate-const_panic/feature-gate-const_panic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/feature-gate-const_panic.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/feature-gate-const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/feature-gate-const_panic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/feature-gate-const_panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
error[E0658]: panicking in constants is unstable
  --> /checkout/src/test/ui/consts/const-eval/feature-gate-const_panic.rs:3:15
   |
LL | const Z: () = panic!("cheese");
   |
   = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
   = help: add `#![feature(const_panic)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: panicking in constants is unstable
  --> /checkout/src/test/ui/consts/const-eval/feature-gate-const_panic.rs:6:15
   |
LL | const Y: () = unreachable!();
   |
   = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
   = help: add `#![feature(const_panic)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0658]: panicking in constants is unstable
  --> /checkout/src/test/ui/consts/const-eval/feature-gate-const_panic.rs:9:15
   |
LL | const X: () = unimplemented!();
   |
   = note: see issue #51999 <https://github.com/rust-lang/rust/issues/51999> for more information
   = help: add `#![feature(const_panic)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
---
diff of stderr:

5    |     ^^^^^^^^
6    |     |
7    |     the evaluated program panicked at 'explicit panic', $DIR/unwind-abort.rs:5:5
-    |     inside `foo` at $SRC_DIR/std/src/macros.rs:LL:COL
+    |     inside `foo` at $SRC_DIR/std/src/panic.rs:LL:COL
9    |     inside `_` at $DIR/unwind-abort.rs:8:15
10 ...
11 LL | const _: () = foo();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/unwind-abort/unwind-abort.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/unwind-abort/unwind-abort.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/unwind-abort.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/unwind-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/unwind-abort" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/unwind-abort/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |     panic!() //~ ERROR any use of this value will cause an error [const_err]
   |     |
   |     the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/unwind-abort.rs:5:5
   |     inside `foo` at /checkout/library/std/src/panic.rs:27:9
   |     inside `foo` at /checkout/library/std/src/panic.rs:27:9
   |     inside `_` at /checkout/src/test/ui/consts/const-eval/unwind-abort.rs:8:15
...
LL | const _: () = foo();
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-unwrap.rs stdout ----
diff of stderr:

5    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |                     |
7    |                     the evaluated program panicked at 'called `Option::unwrap()` on a `None` value', $DIR/const-unwrap.rs:9:38
-    |                     inside `Option::<i32>::unwrap` at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+    |                     inside `Option::<i32>::unwrap` at $SRC_DIR/core/src/panic.rs:LL:COL
9    |                     inside `BAR` at $DIR/const-unwrap.rs:9:18
11   ::: $DIR/const-unwrap.rs:9:1


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unwrap/const-unwrap.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-unwrap.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-unwrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unwrap" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unwrap/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |             None => panic!("called `Option::unwrap()` on a `None` value"),
   |                     |
   |                     |
   |                     the evaluated program panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/test/ui/consts/const-unwrap.rs:9:38
   |                     inside `Option::<i32>::unwrap` at /checkout/library/core/src/panic.rs:18:9
   |                     inside `BAR` at /checkout/src/test/ui/consts/const-unwrap.rs:9:18
  ::: /checkout/src/test/ui/consts/const-unwrap.rs:9:1
   |
   |
LL | const BAR: i32 = Option::<i32>::None.unwrap(); //~ NOTE
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs stdout ----

error: /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32: unexpected note: 'in this expansion of panic!'
error: 1 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/auxiliary"
    Error {
        line_num: 32,
        kind: Some(
            Note,
            Note,
        ),
        msg: "in this expansion of panic!",
]

thread '[ui] ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
test result: FAILED. 11155 passed; 4 failed; 86 ignored; 0 measured; 0 filtered out; finished in 138.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:53
