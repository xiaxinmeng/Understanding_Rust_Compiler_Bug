plain
.................................................................................................... 9000/11208
.................................................................................................... 9100/11208
..................................................................................................i. 9200/11208
.....i.............................................................................................. 9300/11208
.....................................iiiiii..iiiiii.i............................................... 9400/11208
.................................................................................................... 9600/11208
.................................................................................................... 9700/11208
.................................................................................................... 9800/11208
.................................................................................................... 9900/11208
---
i................................................................................................... 11200/11208
........
failures:

---- [ui] ui/closures/2229_closure_analysis/by_value.rs stdout ----

1 error[E0658]: attributes on expressions are experimental
-   --> $DIR/by_value.rs:20:13
+   --> $DIR/by_value.rs:22:13
+   --> $DIR/by_value.rs:22:13
3    |
4 LL |     let c = #[rustc_capture_analysis]

17    = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information
18 
18 
19 error: First Pass analysis includes:
-   --> $DIR/by_value.rs:23:5
+   --> $DIR/by_value.rs:25:5
22 LL | /     || {
23 LL | |

29    | |_____^
29    | |_____^
30    |
31 note: Capturing t[(0, 0),Deref,(0, 0)] -> ByValue
-   --> $DIR/by_value.rs:26:17
+   --> $DIR/by_value.rs:28:17
34 LL |         let p = t.0.0;
35    |                 ^^^^^


36 note: Capturing t[(1, 0)] -> ImmBorrow
-   --> $DIR/by_value.rs:29:29
+   --> $DIR/by_value.rs:31:29
38    |
39 LL |         println!("{} {:?}", t.1, p);

41 
41 
42 error: Min Capture analysis includes:
-   --> $DIR/by_value.rs:23:5
+   --> $DIR/by_value.rs:25:5
45 LL | /     || {
46 LL | |

52    | |_____^
52    | |_____^
53    |
54 note: Min Capture t[(0, 0)] -> ByValue
-   --> $DIR/by_value.rs:26:17
+   --> $DIR/by_value.rs:28:17
57 LL |         let p = t.0.0;
58    |                 ^^^^^


59 note: Min Capture t[(1, 0)] -> ImmBorrow
-   --> $DIR/by_value.rs:29:29
+   --> $DIR/by_value.rs:31:29
61    |
62 LL |         println!("{} {:?}", t.1, p);


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/by_value/by_value.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/by_value.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/by_value" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/by_value/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: attributes on expressions are experimental
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:22:13
   |
LL |     let c = #[rustc_capture_analysis]
   |
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable

warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:5:12
   |
LL | #![feature(capture_disjoint_fields)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information


error: First Pass analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:25:5
LL | /     || {
LL | /     || {
LL | |     //~^ First Pass analysis includes:
LL | |     //~| Min Capture analysis includes:
LL | |         let p = t.0.0;
...  |
LL | |         //~| NOTE: Min Capture t[(1, 0)] -> ImmBorrow
LL | |     };
   |
   |
note: Capturing t[(0, 0),Deref,(0, 0)] -> ByValue
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:28:17
LL |         let p = t.0.0;
   |                 ^^^^^
   |                 ^^^^^
note: Capturing t[(1, 0)] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:31:29
   |
LL |         println!("{} {:?}", t.1, p);


error: Min Capture analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:25:5
LL | /     || {
LL | /     || {
LL | |     //~^ First Pass analysis includes:
LL | |     //~| Min Capture analysis includes:
LL | |         let p = t.0.0;
...  |
LL | |         //~| NOTE: Min Capture t[(1, 0)] -> ImmBorrow
LL | |     };
   |
   |
note: Min Capture t[(0, 0)] -> ByValue
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:28:17
LL |         let p = t.0.0;
   |                 ^^^^^
   |                 ^^^^^
note: Min Capture t[(1, 0)] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/by_value.rs:31:29
   |
LL |         println!("{} {:?}", t.1, p);

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0658`.
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:31
