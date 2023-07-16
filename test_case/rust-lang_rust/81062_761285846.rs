plain
.................................................................................................... 900/11263
.................................................................................................... 1000/11263
............................................i....................................................... 1100/11263
.................................................................................................... 1200/11263
.....................................F..F.F......................................................... 1300/11263
.................................................................................................... 1500/11263
.................................................................................................... 1600/11263
.................................................................................................... 1700/11263
.................................................................................................... 1800/11263
---
......................................................i............................................. 11200/11263
...............................................................
failures:

---- [ui] ui/closures/2229_closure_analysis/capture-analysis-1.rs stdout ----

1 error[E0658]: attributes on expressions are experimental
-   --> $DIR/capture-analysis-1.rs:18:13
+   --> $DIR/capture-analysis-1.rs:17:13
+   --> $DIR/capture-analysis-1.rs:17:13
3    |
4 LL |     let c = #[rustc_capture_analysis]


8    = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable
9 
10 warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
-   --> $DIR/capture-analysis-1.rs:2:12
+   --> $DIR/capture-analysis-1.rs:1:12
12    |
13 LL | #![feature(capture_disjoint_fields)]

17    = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information
18 
18 
19 error: First Pass analysis includes:
-   --> $DIR/capture-analysis-1.rs:21:5
+   --> $DIR/capture-analysis-1.rs:20:5
22 LL | /     || {
23 LL | |

29    | |_____^
29    | |_____^
30    |
31 note: Capturing p[] -> ImmBorrow
-   --> $DIR/capture-analysis-1.rs:24:26
+   --> $DIR/capture-analysis-1.rs:23:26
33    |
34 LL |         println!("{:?}", p);


36 note: Capturing p[(0, 0)] -> ImmBorrow
-   --> $DIR/capture-analysis-1.rs:27:26
+   --> $DIR/capture-analysis-1.rs:26:26
38    |
39 LL |         println!("{:?}", p.x);


41 note: Capturing q[(0, 0)] -> ImmBorrow
-   --> $DIR/capture-analysis-1.rs:30:26
+   --> $DIR/capture-analysis-1.rs:29:26
43    |
44 LL |         println!("{:?}", q.x);


46 note: Capturing q[] -> ImmBorrow
-   --> $DIR/capture-analysis-1.rs:32:26
+   --> $DIR/capture-analysis-1.rs:31:26
48    |
49 LL |         println!("{:?}", q);

51 
51 
52 error: Min Capture analysis includes:
-   --> $DIR/capture-analysis-1.rs:21:5
+   --> $DIR/capture-analysis-1.rs:20:5
55 LL | /     || {
56 LL | |

62    | |_____^
62    | |_____^
63    |
64 note: Min Capture p[] -> ImmBorrow
-   --> $DIR/capture-analysis-1.rs:24:26
+   --> $DIR/capture-analysis-1.rs:23:26
66    |
67 LL |         println!("{:?}", p);


69 note: Min Capture q[] -> ImmBorrow
-   --> $DIR/capture-analysis-1.rs:32:26
+   --> $DIR/capture-analysis-1.rs:31:26
71    |
72 LL |         println!("{:?}", q);


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/capture-analysis-1/capture-analysis-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/capture-analysis-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/capture-analysis-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/capture-analysis-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: attributes on expressions are experimental
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:17:13
   |
LL |     let c = #[rustc_capture_analysis]
   |
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable

warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:1:12
   |
LL | #![feature(capture_disjoint_fields)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information


error: First Pass analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:20:5
LL | /     || {
LL | /     || {
LL | |     //~^ First Pass analysis includes:
LL | |     //~| Min Capture analysis includes:
LL | |         println!("{:?}", p);
...  |
LL | |         //~| NOTE: Min Capture q[] -> ImmBorrow
LL | |     };
   |
   |
note: Capturing p[] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:23:26
   |
LL |         println!("{:?}", p);
   |                          ^
note: Capturing p[(0, 0)] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:26:26
   |
LL |         println!("{:?}", p.x);
   |                          ^^^
note: Capturing q[(0, 0)] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:29:26
   |
LL |         println!("{:?}", q.x);
   |                          ^^^
note: Capturing q[] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:31:26
   |
LL |         println!("{:?}", q);


error: Min Capture analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:20:5
LL | /     || {
LL | /     || {
LL | |     //~^ First Pass analysis includes:
LL | |     //~| Min Capture analysis includes:
LL | |         println!("{:?}", p);
...  |
LL | |         //~| NOTE: Min Capture q[] -> ImmBorrow
LL | |     };
   |
   |
note: Min Capture p[] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:23:26
   |
LL |         println!("{:?}", p);
   |                          ^
note: Min Capture q[] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-1.rs:31:26
   |
LL |         println!("{:?}", q);

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.

------------------------------------------


---- [ui] ui/closures/2229_closure_analysis/capture-analysis-2.rs stdout ----

1 error[E0658]: attributes on expressions are experimental
-   --> $DIR/capture-analysis-2.rs:17:13
+   --> $DIR/capture-analysis-2.rs:16:13
+   --> $DIR/capture-analysis-2.rs:16:13
3    |
4 LL |     let c = #[rustc_capture_analysis]


8    = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable
9 
10 warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
-   --> $DIR/capture-analysis-2.rs:2:12
+   --> $DIR/capture-analysis-2.rs:1:12
12    |
13 LL | #![feature(capture_disjoint_fields)]

17    = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information
18 
18 
19 error: First Pass analysis includes:
-   --> $DIR/capture-analysis-2.rs:20:5
+   --> $DIR/capture-analysis-2.rs:19:5
22 LL | /     || {
23 LL | |

29    | |_____^
29    | |_____^
30    |
31 note: Capturing p[(0, 0)] -> ByValue
-   --> $DIR/capture-analysis-2.rs:23:18
+   --> $DIR/capture-analysis-2.rs:22:18
33    |
34 LL |         let _x = p.x;


36 note: Capturing p[] -> ImmBorrow
-   --> $DIR/capture-analysis-2.rs:26:26
+   --> $DIR/capture-analysis-2.rs:25:26
38    |
39 LL |         println!("{:?}", p);

41 
41 
42 error: Min Capture analysis includes:
-   --> $DIR/capture-analysis-2.rs:20:5
+   --> $DIR/capture-analysis-2.rs:19:5
45 LL | /     || {
46 LL | |

52    | |_____^
52    | |_____^
53    |
54 note: Min Capture p[] -> ByValue
-   --> $DIR/capture-analysis-2.rs:23:18
+   --> $DIR/capture-analysis-2.rs:22:18
56    |
57 LL |         let _x = p.x;
58    |                  ^^^ p[] captured as ByValue here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/capture-analysis-2/capture-analysis-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/capture-analysis-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/capture-analysis-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/capture-analysis-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: attributes on expressions are experimental
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs:16:13
   |
LL |     let c = #[rustc_capture_analysis]
   |
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable

warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs:1:12
   |
LL | #![feature(capture_disjoint_fields)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information


error: First Pass analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs:19:5
LL | /     || {
LL | /     || {
LL | |     //~^ First Pass analysis includes:
LL | |     //~| Min Capture analysis includes:
LL | |         let _x = p.x;
...  |
LL | |         //~| NOTE: p[] used here
LL | |     };
   |
   |
note: Capturing p[(0, 0)] -> ByValue
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs:22:18
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |         let _x = p.x;
   |                  ^^^
note: Capturing p[] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs:25:26
   |
LL |         println!("{:?}", p);


error: Min Capture analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs:19:5
LL | /     || {
LL | /     || {
LL | |     //~^ First Pass analysis includes:
LL | |     //~| Min Capture analysis includes:
LL | |         let _x = p.x;
...  |
LL | |         //~| NOTE: p[] used here
LL | |     };
   |
   |
note: Min Capture p[] -> ByValue
  --> /checkout/src/test/ui/closures/2229_closure_analysis/capture-analysis-2.rs:22:18
   |
LL |         let _x = p.x;
   |                  ^^^ p[] captured as ByValue here
...
LL |         println!("{:?}", p);
   |                          ^ p[] used here
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs stdout ----

1 error[E0658]: attributes on expressions are experimental
1 error[E0658]: attributes on expressions are experimental
-   --> $DIR/deep-multilevel-tuple.rs:12:13
+   --> $DIR/deep-multilevel-tuple.rs:11:13
3    |
4 LL |     let c = #[rustc_capture_analysis]


8    = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable
9 
10 warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
-   --> $DIR/deep-multilevel-tuple.rs:2:12
+   --> $DIR/deep-multilevel-tuple.rs:1:12
12    |
13 LL | #![feature(capture_disjoint_fields)]

17    = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information
18 
18 
19 error: First Pass analysis includes:
-   --> $DIR/deep-multilevel-tuple.rs:15:5
+   --> $DIR/deep-multilevel-tuple.rs:14:5
22 LL | /     || {
23 LL | |

29    | |_____^
29    | |_____^
30    |
31 note: Capturing t[(0, 0),(0, 0),(0, 0)] -> ImmBorrow
-   --> $DIR/deep-multilevel-tuple.rs:18:18
+   --> $DIR/deep-multilevel-tuple.rs:17:18
33    |
34 LL |         let x = &t.0.0.0;


36 note: Capturing t[(1, 0),(1, 0),(1, 0)] -> MutBorrow
-   --> $DIR/deep-multilevel-tuple.rs:20:9
+   --> $DIR/deep-multilevel-tuple.rs:19:9
38    |
39 LL |         t.1.1.1 = 9;


41 note: Capturing t[] -> ImmBorrow
-   --> $DIR/deep-multilevel-tuple.rs:23:26
+   --> $DIR/deep-multilevel-tuple.rs:22:26
43    |
44 LL |         println!("{:?}", t);

46 
46 
47 error: Min Capture analysis includes:
-   --> $DIR/deep-multilevel-tuple.rs:15:5
+   --> $DIR/deep-multilevel-tuple.rs:14:5
50 LL | /     || {
51 LL | |

57    | |_____^
57    | |_____^
58    |
59 note: Min Capture t[] -> MutBorrow
-   --> $DIR/deep-multilevel-tuple.rs:20:9
+   --> $DIR/deep-multilevel-tuple.rs:19:9
61    |
62 LL |         t.1.1.1 = 9;
63    |         ^^^^^^^ t[] captured as MutBorrow here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple/deep-multilevel-tuple.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/deep-multilevel-tuple.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: attributes on expressions are experimental
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:11:13
   |
LL |     let c = #[rustc_capture_analysis]
   |
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = note: see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
   = help: add `#![feature(stmt_expr_attributes)]` to the crate attributes to enable

warning: the feature `capture_disjoint_fields` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:1:12
   |
LL | #![feature(capture_disjoint_fields)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #53488 <https://github.com/rust-lang/rust/issues/53488> for more information


error: First Pass analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:14:5
LL | /     || {
LL | /     || {
LL | |     //~^ ERROR: First Pass analysis includes:
LL | |     //~| ERROR: Min Capture analysis includes:
LL | |         let x = &t.0.0.0;
...  |
LL | |         //~| NOTE: t[] used here
LL | |     };
   |
   |
note: Capturing t[(0, 0),(0, 0),(0, 0)] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:17:18
   |
LL |         let x = &t.0.0.0;
   |                  ^^^^^^^
note: Capturing t[(1, 0),(1, 0),(1, 0)] -> MutBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:19:9
   |
LL |         t.1.1.1 = 9;
   |         ^^^^^^^
note: Capturing t[] -> ImmBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:22:26
   |
LL |         println!("{:?}", t);


error: Min Capture analysis includes:
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:14:5
LL | /     || {
LL | /     || {
LL | |     //~^ ERROR: First Pass analysis includes:
LL | |     //~| ERROR: Min Capture analysis includes:
LL | |         let x = &t.0.0.0;
...  |
LL | |         //~| NOTE: t[] used here
LL | |     };
   |
   |
note: Min Capture t[] -> MutBorrow
  --> /checkout/src/test/ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs:19:9
   |
LL |         t.1.1.1 = 9;
   |         ^^^^^^^ t[] captured as MutBorrow here
...
LL |         println!("{:?}", t);
   |                          ^ t[] used here
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0658`.


------------------------------------------



failures:
    [ui] ui/closures/2229_closure_analysis/capture-analysis-1.rs
    [ui] ui/closures/2229_closure_analysis/capture-analysis-2.rs
    [ui] ui/closures/2229_closure_analysis/deep-multilevel-tuple.rs
test result: FAILED. 11174 passed; 3 failed; 86 ignored; 0 measured; 0 filtered out; finished in 119.60s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:53
