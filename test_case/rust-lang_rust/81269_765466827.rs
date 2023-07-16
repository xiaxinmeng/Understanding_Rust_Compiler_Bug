plain
................ii................i.i..ii........................................................... 7100/11277
.................................................................................................... 7200/11277
.................................................................................................... 7300/11277
.................................................................................................... 7400/11277
...............................................F...F...........i..ii................................ 7500/11277
...................................................i................................................ 7700/11277
....................................i............................................................... 7800/11277
............................i....................................................................... 7900/11277
.................................................................................................... 8000/11277
---
---- [ui] ui/no-capture-arc.rs stdout ----
diff of stderr:

14    |
15    = note: borrow occurs due to deref coercion to `Vec<i32>`
16 note: deref defined here
-   --> $SRC_DIR/alloc/src/sync.rs:LL:COL
+   --> $SRC_DIR/alloc/src/sync/arc.rs:LL:COL
19 LL |     type Target = T;
20    |     ^^^^^^^^^^^^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-capture-arc/no-capture-arc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args no-capture-arc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-capture-arc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-capture-arc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-capture-arc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `arc_v`
   |
   |
LL |     let arc_v = Arc::new(v);
   |         ----- move occurs because `arc_v` has type `Arc<Vec<i32>>`, which does not implement the `Copy` trait
LL | 
LL |     thread::spawn(move|| {
   |                   ------ value moved into closure here
LL |         assert_eq!((*arc_v)[3], 4);
   |                      ----- variable moved due to use in closure
...
LL |     assert_eq!((*arc_v)[2], 3);
   |                ^^^^^^^^ value borrowed here after move
   |
   = note: borrow occurs due to deref coercion to `Vec<i32>`
note: deref defined here
  --> /checkout/library/alloc/src/sync/arc.rs:1248:5
LL |     type Target = T;
   |     ^^^^^^^^^^^^^^^^

error: aborting due to previous error
---
---- [ui] ui/no-reuse-move-arc.rs stdout ----
diff of stderr:

14    |
15    = note: borrow occurs due to deref coercion to `Vec<i32>`
16 note: deref defined here
-   --> $SRC_DIR/alloc/src/sync.rs:LL:COL
+   --> $SRC_DIR/alloc/src/sync/arc.rs:LL:COL
19 LL |     type Target = T;
20    |     ^^^^^^^^^^^^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-reuse-move-arc/no-reuse-move-arc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args no-reuse-move-arc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-reuse-move-arc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-reuse-move-arc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-reuse-move-arc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: borrow of moved value: `arc_v`
   |
   |
LL |     let arc_v = Arc::new(v);
   |         ----- move occurs because `arc_v` has type `Arc<Vec<i32>>`, which does not implement the `Copy` trait
LL | 
LL |     thread::spawn(move|| {
   |                   ------ value moved into closure here
LL |         assert_eq!((*arc_v)[3], 4);
   |                      ----- variable moved due to use in closure
...
LL |     assert_eq!((*arc_v)[2], 3); //~ ERROR borrow of moved value: `arc_v`
   |                ^^^^^^^^ value borrowed here after move
   |
   = note: borrow occurs due to deref coercion to `Vec<i32>`
note: deref defined here
  --> /checkout/library/alloc/src/sync/arc.rs:1248:5
LL |     type Target = T;
   |     ^^^^^^^^^^^^^^^^

error: aborting due to previous error
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:04
