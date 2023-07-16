plain
.................................................................................................... 2300/11252
.................................................................................................... 2400/11252
.............................................................i..i................................... 2500/11252
.................................................................................................... 2600/11252
.........................F......................................iiiii............................... 2700/11252
.................................................................................................... 2900/11252
.................................................................................................... 3000/11252
.................................................................................................... 3100/11252
.................................................................................................... 3200/11252
---
.................................................................................................... 9000/11252
.................................................................................................... 9100/11252
.................................................................................................... 9200/11252
...............................................i......i............................................. 9300/11252
......................................................................................iiiiii..iiiiii 9400/11252
.................................................................................................... 9600/11252
.................................................................................................... 9700/11252
.................................................................................................... 9800/11252
.................................................................................................... 9900/11252
---
---- [ui] ui/destructuring-assignment/tuple_struct_destructure_fail.rs stdout ----
diff of stderr:

36    |
37 LL |     TupleStruct(_, _) = TupleStruct(1, 2);
38    |                  ^^^
+ help: use `..` to ignore all fields
+    |
+ LL |     TupleStruct(..) = TupleStruct(1, 2);
39 
39 
40 error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
41   --> $DIR/tuple_struct_destructure_fail.rs:34:5
59    |
59    |
60 LL |     Enum::SingleVariant(_, _) = Enum::SingleVariant(1, 2);
61    |                          ^^^
+ help: use `..` to ignore all fields
+    |
+ LL |     Enum::SingleVariant(..) = Enum::SingleVariant(1, 2);
62 
62 
63 error[E0070]: invalid left-hand side of assignment
64   --> $DIR/tuple_struct_destructure_fail.rs:40:12

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
   |     ^^^^^^^^^^^^^^ expected 2 fields, found 1
   |
help: use `_` to explicitly ignore each field
   |
LL |     TupleStruct(_, _) = TupleStruct(1, 2);
help: use `..` to ignore all fields
   |
   |
LL |     TupleStruct(..) = TupleStruct(1, 2);


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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     SingleVariant(S, T)
   |     ------------------- tuple variant defined here
...
LL |     Enum::SingleVariant(_) = Enum::SingleVariant(1, 2);
   |     ^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
   |
help: use `_` to explicitly ignore each field
   |
LL |     Enum::SingleVariant(_, _) = Enum::SingleVariant(1, 2);
help: use `..` to ignore all fields
   |
   |
LL |     Enum::SingleVariant(..) = Enum::SingleVariant(1, 2);


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
---- [ui] ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs stdout ----
diff of stderr:

22    |
23 LL |     let P(_) = U {};
24    |           ^
+ help: use `..` to ignore all fields
+    |
+ LL |     let P(..) = U {};
25 
26 error: aborting due to 2 previous errors
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields/issue-67037-pat-tup-scrut-ty-diff-less-fields.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-67037-pat-tup-scrut-ty-diff-less-fields.rs:19:9
   |
LL |     let P() = U {}; //~ ERROR mismatched types
   |         ^^^   ---- this expression has type `U`
   |         |
   |         expected struct `U`, found struct `P`
   = note: expected struct `U`
   = note: expected struct `U`
              found struct `P<_>`

error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 1 field
   |
   |
LL | struct P<T>(T); // 1 type parameter wanted
   | --------------- tuple struct defined here
...
LL |     let P() = U {}; //~ ERROR mismatched types
   |         ^^^ expected 1 field, found 0
   |
help: use `_` to explicitly ignore each field
   |
LL |     let P(_) = U {}; //~ ERROR mismatched types
help: use `..` to ignore all fields
   |
   |
LL |     let P(..) = U {}; //~ ERROR mismatched types

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0023, E0308.
---
test result: FAILED. 11164 passed; 2 failed; 86 ignored; 0 measured; 0 filtered out; finished in 137.59s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:08
