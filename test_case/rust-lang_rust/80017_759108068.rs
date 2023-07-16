plain
.................................................................................................... 2300/11251
.................................................................................................... 2400/11251
.............................................................i..i................................... 2500/11251
.................................................................................................... 2600/11251
..........................F.....................................iiiii............................... 2700/11251
.................................................................................................... 2900/11251
.................................................................................................... 3000/11251
.................................................................................................... 3100/11251
.................................................................................................... 3200/11251
---
.................................................................................................... 9000/11251
.................................................................................................... 9100/11251
.................................................................................................... 9200/11251
..............................................i......i.............................................. 9300/11251
.....................................................................................iiiiii..iiiiii. 9400/11251
.................................................................................................... 9600/11251
.................................................................................................... 9700/11251
.................................................................................................... 9800/11251
.................................................................................................... 9900/11251
---
diff of stderr:

30    | ------------------------------- tuple struct defined here
31 ...
32 LL |     TupleStruct(_) = TupleStruct(1, 2);
-    |     ^^^^^^^^^^^^^^ expected 2 fields, found 1
-    |
- help: use `_` to explicitly ignore each field
-    |
- LL |     TupleStruct(_, _) = TupleStruct(1, 2);
-    |                  ^^^
- help: use `..` to ignore all fields
-    |
- LL |     TupleStruct(..) = TupleStruct(1, 2);
+    |     ^^^^^^^^^^^^^-
+    |     |            |
+    |     |            |
+    |     |            help: use `_` to explicitly ignore each field
+    |     expected 2 fields, found 1
43 
44 error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
45   --> $DIR/tuple_struct_destructure_fail.rs:34:5

57    |     ------------------- tuple variant defined here
58 ...
59 LL |     Enum::SingleVariant(_) = Enum::SingleVariant(1, 2);
-    |     ^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
-    |
- help: use `_` to explicitly ignore each field
-    |
- LL |     Enum::SingleVariant(_, _) = Enum::SingleVariant(1, 2);
-    |                          ^^^
- help: use `..` to ignore all fields
-    |
- LL |     Enum::SingleVariant(..) = Enum::SingleVariant(1, 2);
+    |     ^^^^^^^^^^^^^^^^^^^^^-
+    |     |                    |
+    |     |                    |
+    |     |                    help: use `_` to explicitly ignore each field
+    |     expected 2 fields, found 1
70 
71 error[E0070]: invalid left-hand side of assignment
72   --> $DIR/tuple_struct_destructure_fail.rs:40:12

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_struct_destructure_fail/tuple_struct_destructure_fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args destructuring-assignment/tuple_struct_destructure_fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_struct_destructure_fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/tuple_struct_destructure_fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `..` can only be used once per tuple struct or variant pattern
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:25:27
   |
LL |     TupleStruct(a, .., b, ..) = TupleStruct(0, 1);
   |                    --     ^^ can only be used once per tuple struct or variant pattern
   |                    previously used here


error: `..` can only be used once per tuple struct or variant pattern
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:27:35
   |
LL |     Enum::SingleVariant(a, .., b, ..) = Enum::SingleVariant(0, 1);
   |                            --     ^^ can only be used once per tuple struct or variant pattern
   |                            previously used here


error[E0023]: this pattern has 3 fields, but the corresponding tuple struct has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:30:5
   |
LL | struct TupleStruct<S, T>(S, T);
   | ------------------------------- tuple struct defined here
...
LL |     TupleStruct(a, a, b) = TupleStruct(1, 2);
   |     ^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:32:5
   |
LL | struct TupleStruct<S, T>(S, T);
   | ------------------------------- tuple struct defined here
...
LL |     TupleStruct(_) = TupleStruct(1, 2);
   |     |            |
   |     |            |
   |     |            help: use `_` to explicitly ignore each field
   |     expected 2 fields, found 1

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:34:5
   |
LL |     SingleVariant(S, T)
   |     ------------------- tuple variant defined here
...
LL |     Enum::SingleVariant(a, a, b) = Enum::SingleVariant(1, 2);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:36:5
   |
LL |     SingleVariant(S, T)
   |     ------------------- tuple variant defined here
...
LL |     Enum::SingleVariant(_) = Enum::SingleVariant(1, 2);
   |     |                    |
   |     |                    |
   |     |                    help: use `_` to explicitly ignore each field
   |     expected 2 fields, found 1

error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:40:12
   |
LL |     test() = TupleStruct(0, 0);
   |     ------ ^
   |     cannot assign to this expression


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:42:14
   |
LL |     (test)() = TupleStruct(0, 0);
   |     -------- ^
   |     cannot assign to this expression


error[E0070]: invalid left-hand side of assignment
  --> /checkout/src/test/ui/destructuring-assignment/tuple_struct_destructure_fail.rs:44:38
   |
LL |     <Alias::<isize> as Test>::test() = TupleStruct(0, 0);
   |     -------------------------------- ^
   |     cannot assign to this expression

error: aborting due to 9 previous errors

---

---- [ui] ui/pattern/pat-tuple-underfield.rs stdout ----
diff of stderr:

1 error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E::S`
-   --> $DIR/pat-tuple-underfield.rs:45:9
3    |
3    |
4 LL |     S(i32, f32),
5    |     ----------- `E::S` defined here
20    |         expected 2 fields, found 1
21 
21 
22 error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:14:9
24    |
24    |
25 LL | struct S(i32, f32);
26    | ------------------- tuple struct defined here
27 ...
27 ...
28 LL |         S(_) => {}
-    |         ^^^^ expected 2 fields, found 1
-    |
- help: use `_` to explicitly ignore each field
-    |
- LL |         S(_, _) => {}
-    |            ^^^
- help: use `..` to ignore all fields
-    |
- LL |         S(..) => {}
+    |         ^^^-
+    |         |  |
+    |         |  |
+    |         |  help: use `_` to explicitly ignore each field
+    |         expected 2 fields, found 1
39 
40 error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:20:9
42    |
42    |
43 LL | struct S(i32, f32);
44    | ------------------- tuple struct defined here
56    |           ^^
57 
57 
58 error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:27:9
60    |
60    |
61 LL |     S(i32, f32),
62    |     ----------- tuple variant defined here
68    |         expected 2 fields, found 1
69 
69 
70 error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:33:9
72    |
72    |
73 LL |     S(i32, f32),
74    |     ----------- tuple variant defined here
75 ...
75 ...
76 LL |         E::S(_) => {}
-    |         ^^^^^^^ expected 2 fields, found 1
-    |
- help: use `_` to explicitly ignore each field
-    |
- LL |         E::S(_, _) => {}
-    |               ^^^
- help: use `..` to ignore all fields
-    |
- LL |         E::S(..) => {}
+    |         ^^^^^^-
+    |         |     |
+    |         |     |
+    |         |     help: use `_` to explicitly ignore each field
+    |         expected 2 fields, found 1
87 
88 error[E0023]: this pattern has 0 fields, but the corresponding tuple variant has 2 fields
-   --> $DIR/pat-tuple-underfield.rs:39:9
90    |
90    |
91 LL |     S(i32, f32),
92    |     ----------- tuple variant defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield/pat-tuple-underfield.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield/pat-tuple-underfield.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-tuple-underfield.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-tuple-underfield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-underfield/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E::S`
   |
   |
LL |     S(i32, f32),
   |     ----------- `E::S` defined here
...
LL |         E::S => {}
   |         ^^^^ help: use the tuple variant pattern syntax instead: `E::S(_, _)`

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
   |
   |
LL | struct S(i32, f32);
   | ------------------- tuple struct defined here
...
LL |         S(x) => {}
   |         ^^^-
   |         |  |
   |         |  help: use `_` to explicitly ignore each field
   |         expected 2 fields, found 1

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 2 fields
   |
   |
LL | struct S(i32, f32);
   | ------------------- tuple struct defined here
...
LL |         S(_) => {}
   |         ^^^-
   |         |  |
   |         |  help: use `_` to explicitly ignore each field
   |         expected 2 fields, found 1

error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 2 fields
   |
   |
LL | struct S(i32, f32);
   | ------------------- tuple struct defined here
...
LL |         S() => {}
   |         ^^^ expected 2 fields, found 0
   |
help: use `_` to explicitly ignore each field
   |
LL |         S(_, _) => {}
help: use `..` to ignore all fields
   |
   |
LL |         S(..) => {}


error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   |
   |
LL |     S(i32, f32),
   |     ----------- tuple variant defined here
...
LL |         E::S(x) => {}
   |         |     |
   |         |     |
   |         |     help: use `_` to explicitly ignore each field
   |         expected 2 fields, found 1

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   |
   |
LL |     S(i32, f32),
   |     ----------- tuple variant defined here
...
LL |         E::S(_) => {}
   |         |     |
   |         |     |
   |         |     help: use `_` to explicitly ignore each field
   |         expected 2 fields, found 1

error[E0023]: this pattern has 0 fields, but the corresponding tuple variant has 2 fields
   |
   |
LL |     S(i32, f32),
   |     ----------- tuple variant defined here
...
LL |         E::S() => {}
   |         ^^^^^^ expected 2 fields, found 0
   |
help: use `_` to explicitly ignore each field
   |
LL |         E::S(_, _) => {}
help: use `..` to ignore all fields
   |
   |
LL |         E::S(..) => {}

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0023, E0532.
---
test result: FAILED. 11163 passed; 2 failed; 86 ignored; 0 measured; 0 filtered out; finished in 140.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:59
