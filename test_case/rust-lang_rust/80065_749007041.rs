plain
.................................................................................................... 3300/11197
.................................................................................................... 3400/11197
.................................................................................................... 3500/11197
.................................................................................................... 3600/11197
.......................................F...F.F...................................................... 3700/11197
.................................................................................................... 3900/11197
.................................................................................................... 4000/11197
.................................................................................................... 4100/11197
.........ii....................................................................................i.... 4200/11197
---
.................................................................................................... 9000/11197
.................................................................................................... 9100/11197
.......................................................................................i......i..... 9200/11197
.................................................................................................... 9300/11197
..........................iiiiii..iiiiii.i.......................................................... 9400/11197
.................................................................................................... 9600/11197
.................................................................................................... 9700/11197
.................................................................................................... 9800/11197
.................................................................................................... 9900/11197
---
diff of stderr:

10   --> $DIR/trait-path-expressions.rs:19:30
11    |
12 LL |   fn f2<'a>(arg : Box<dyn X< { 1 } = 32 >>) {}
+    |                           ---^^^^^
+    |                           |
+    |                           |
+    |                           help: possibly missing closing angle bracket: `try to use the following: X< { 1 }>`
14 
15 warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-expressions/trait-path-expressions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-expressions/trait-path-expressions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/parse/trait-path-expressions.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/parse/trait-path-expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-expressions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-expressions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected expression, found `)`
  --> /checkout/src/test/ui/generic-associated-types/parse/trait-path-expressions.rs:9:39
   |
LL |   fn f1<'a>(arg : Box<dyn X< 1 = 32 >>) {}
   |                              -        ^ expected expression
   |                              while parsing a const generic argument starting here


error: only types can be used in associated type constraints
   |
   |
LL |   fn f2<'a>(arg : Box<dyn X< { 1 } = 32 >>) {}
   |                           ---^^^^^
   |                           |
   |                           help: possibly missing closing angle bracket: `try to use the following: X< { 1 }>`

warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
---
diff of stderr:

2   --> $DIR/trait-path-segments.rs:9:31
3    |
4 LL |     fn f1<'a>(arg : Box<dyn X<X::Y = u32>>) {}
+    |                             --^^^^
+    |                             |
+    |                             |
+    |                             help: possibly missing closing angle bracket: `try to use the following: X<X::Y>`
6 
7 error: qualified paths cannot be used in associated type constraints

9    |
9    |
10 LL |     impl<T : X<<Self as X>::Y<'a> = &'a u32>> Z for T {}
-    |                ^^^^^^^^^-^^^^^^^^
-    |                         not allowed in associated type constraints
-    |                         not allowed in associated type constraints
+    |              --^^^^^^^^^-^^^^^^^^
+    |              |          not allowed in associated type constraints
+    |              |          not allowed in associated type constraints
+    |              help: possibly missing closing angle bracket: `try to use the following: X<<Self as X>::Y<'a>>`
14 
15 error: paths with multiple segments cannot be used in associated type constraints

17    |
17    |
18 LL |     impl<T : X<X::Y<'a> = &'a u32>> Z for T {}
+    |              --^^^^^^^^
+    |              |
+    |              |
+    |              help: possibly missing closing angle bracket: `try to use the following: X<X::Y<'a>>`
20 
21 warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-segments/trait-path-segments.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-segments/trait-path-segments.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/parse/trait-path-segments.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/parse/trait-path-segments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-segments" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-segments/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: paths with multiple segments cannot be used in associated type constraints
   |
   |
LL |     fn f1<'a>(arg : Box<dyn X<X::Y = u32>>) {}
   |                             --^^^^
   |                             |
   |                             help: possibly missing closing angle bracket: `try to use the following: X<X::Y>`

error: qualified paths cannot be used in associated type constraints
   |
   |
LL |     impl<T : X<<Self as X>::Y<'a> = &'a u32>> Z for T {}
   |              --^^^^^^^^^-^^^^^^^^
   |              |          not allowed in associated type constraints
   |              |          not allowed in associated type constraints
   |              help: possibly missing closing angle bracket: `try to use the following: X<<Self as X>::Y<'a>>`

error: paths with multiple segments cannot be used in associated type constraints
   |
   |
LL |     impl<T : X<X::Y<'a> = &'a u32>> Z for T {}
   |              --^^^^^^^^
   |              |
   |              help: possibly missing closing angle bracket: `try to use the following: X<X::Y<'a>>`

warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
---
diff of stderr:

2   --> $DIR/trait-path-types.rs:9:29
3    |
4 LL |   fn f<'a>(arg : Box<dyn X< [u8; 1] = u32>>) {}
+    |                          ---^^^^^^^
+    |                          |
+    |                          |
+    |                          help: possibly missing closing angle bracket: `try to use the following: X< [u8; 1]>`
6 
7 error: only path types can be used in associated type constraints

9    |
9    |
10 LL |   fn f1<'a>(arg : Box<dyn X<(Y<'a>) = &'a ()>>) {}
+    |                           --^^^^^^^
+    |                           |
+    |                           |
+    |                           help: possibly missing closing angle bracket: `try to use the following: X<(Y<'a>)>`
12 
13 error: only types can be used in associated type constraints

15    |
15    |
16 LL |   fn f1<'a>(arg : Box<dyn X< 'a = u32 >>) {}
+    |                           ---^^
+    |                           |
+    |                           |
+    |                           help: possibly missing closing angle bracket: `try to use the following: X< 'a>`
18 
19 warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-types/trait-path-types.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-types/trait-path-types.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/parse/trait-path-types.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/parse/trait-path-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/parse/trait-path-types/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: only path types can be used in associated type constraints
   |
   |
LL |   fn f<'a>(arg : Box<dyn X< [u8; 1] = u32>>) {}
   |                          ---^^^^^^^
   |                          |
   |                          help: possibly missing closing angle bracket: `try to use the following: X< [u8; 1]>`

error: only path types can be used in associated type constraints
   |
   |
LL |   fn f1<'a>(arg : Box<dyn X<(Y<'a>) = &'a ()>>) {}
   |                           --^^^^^^^
   |                           |
   |                           help: possibly missing closing angle bracket: `try to use the following: X<(Y<'a>)>`

error: only types can be used in associated type constraints
   |
   |
LL |   fn f1<'a>(arg : Box<dyn X< 'a = u32 >>) {}
   |                           ---^^
   |                           |
   |                           help: possibly missing closing angle bracket: `try to use the following: X< 'a>`

warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(generic_associated_types)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^
   |
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:08
