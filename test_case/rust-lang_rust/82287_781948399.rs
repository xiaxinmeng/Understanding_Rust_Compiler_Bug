plain
.................................................................................................... 9200/11468
.................................................................................................... 9300/11468
.................................................................................................... 9400/11468
..........................i.......i................................................................. 9500/11468
.................................................................iiiiiii..iiiiii.i.................. 9600/11468
.................................................................................................... 9800/11468
.................................................................................................... 9900/11468
.................................................................................................... 10000/11468
.................................................................................................... 10100/11468
---

---- [ui] ui/error-codes/E0027.rs stdout ----
diff of stderr:

43 LL |         Dog { name: x, .. } => {}
45 
45 
- error[E0027]: pattern does not mention fields `name` and `age`
+ error[E0027]: pattern does not mention fields `name`, `age`
47   --> $DIR/E0027.rs:22:9
48    |
49 LL |         Dog {} => {}

-    |         ^^^^^^ missing fields `name` and `age`
+    |         ^^^^^^ missing fields `name`, `age`
52 help: include the missing fields in the pattern
53    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0027/E0027.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0027.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0027.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0027" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0027/auxiliary"
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0027]: pattern does not mention field `name`
  --> /checkout/src/test/ui/error-codes/E0027.rs:11:9
   |
LL |         Dog { age: x } => {} //~ ERROR pattern does not mention field `name`
   |         ^^^^^^^^^^^^^^ missing field `name`
help: include the missing field in the pattern
   |
   |
LL |         Dog { age: x, name } => {} //~ ERROR pattern does not mention field `name`
   |                     ^^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
   |
LL |         Dog { age: x, .. } => {} //~ ERROR pattern does not mention field `name`

error[E0027]: pattern does not mention field `age`
  --> /checkout/src/test/ui/error-codes/E0027.rs:15:9
   |
   |
LL |         Dog { name: x, } => {} //~ ERROR pattern does not mention field `age`
   |         ^^^^^^^^^^^^^^^^ missing field `age`
help: include the missing field in the pattern
   |
   |
LL |         Dog { name: x, age } => {} //~ ERROR pattern does not mention field `age`
   |                      ^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
   |
LL |         Dog { name: x, .. } => {} //~ ERROR pattern does not mention field `age`

error[E0027]: pattern does not mention field `age`
  --> /checkout/src/test/ui/error-codes/E0027.rs:19:9
   |
   |
LL |         Dog { name: x  , } => {} //~ ERROR pattern does not mention field `age`
   |         ^^^^^^^^^^^^^^^^^^ missing field `age`
help: include the missing field in the pattern
   |
   |
LL |         Dog { name: x, age } => {} //~ ERROR pattern does not mention field `age`
   |                      ^^^^^^^
help: if you don't care about this missing field, you can explicitly ignore it
   |
LL |         Dog { name: x, .. } => {} //~ ERROR pattern does not mention field `age`


error[E0027]: pattern does not mention fields `name`, `age`
   |
   |
LL |         Dog {} => {} //~ ERROR pattern does not mention fields `name`, `age`
   |         ^^^^^^ missing fields `name`, `age`
help: include the missing fields in the pattern
   |
   |
LL |         Dog { name, age } => {} //~ ERROR pattern does not mention fields `name`, `age`
   |             ^^^^^^^^^^^^^
help: if you don't care about these missing fields, you can explicitly ignore them
   |
LL |         Dog { .. } => {} //~ ERROR pattern does not mention fields `name`, `age`

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0027`.
For more information about this error, try `rustc --explain E0027`.

------------------------------------------


---- [ui] ui/parser/issue-52496.rs stdout ----

error: /checkout/src/test/ui/parser/issue-52496.rs:8: unexpected error: '8:13: 8:16: missing fields `bar` and `baz` in initializer of `Foo` [E0063]'

error: /checkout/src/test/ui/parser/issue-52496.rs:8: expected error not found: missing fields `bar`, `baz` in initializer of `Foo`
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-52496.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-52496" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-52496/auxiliary"
    Error {
        line_num: 8,
        kind: Some(
            Error,
            Error,
        ),
        msg: "8:13: 8:16: missing fields `bar` and `baz` in initializer of `Foo` [E0063]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 8,
        kind: Some(
            Error,
        ),
        msg: "missing fields `bar`, `baz` in initializer of `Foo`",
]

thread '[ui] ui/parser/issue-52496.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13


---- [ui] ui/structs/struct-pat-derived-error.rs stdout ----
diff of stderr:

10 LL |         let A { x, y } = self.d;
11    |                 ^  ^ struct `A` does not have these fields
12 
- error[E0027]: pattern does not mention fields `b` and `c`
+ error[E0027]: pattern does not mention fields `b`, `c`
15    |
15    |
16 LL |         let A { x, y } = self.d;

-    |             ^^^^^^^^^^ missing fields `b` and `c`
+    |             ^^^^^^^^^^ missing fields `b`, `c`
19 help: include the missing fields in the pattern
20    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-pat-derived-error/struct-pat-derived-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args structs/struct-pat-derived-error.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/struct-pat-derived-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-pat-derived-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-pat-derived-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0609]: no field `d` on type `&A`
  --> /checkout/src/test/ui/structs/struct-pat-derived-error.rs:8:31
   |
LL |         let A { x, y } = self.d; //~ ERROR no field `d` on type `&A`
   |                               ^ help: a field with a similar name exists: `b`

error[E0026]: struct `A` does not have fields named `x`, `y`
   |
   |
LL |         let A { x, y } = self.d; //~ ERROR no field `d` on type `&A`
   |                 ^  ^ struct `A` does not have these fields

error[E0027]: pattern does not mention fields `b`, `c`
   |
   |
LL |         let A { x, y } = self.d; //~ ERROR no field `d` on type `&A`
   |             ^^^^^^^^^^ missing fields `b`, `c`
help: include the missing fields in the pattern
   |
   |
LL |         let A { x, y, b, c } = self.d; //~ ERROR no field `d` on type `&A`
   |                     ^^^^^^^^
help: if you don't care about these missing fields, you can explicitly ignore them
   |
LL |         let A { x, y, .. } = self.d; //~ ERROR no field `d` on type `&A`

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0026, E0027, E0609.
---
test result: FAILED. 11372 passed; 3 failed; 93 ignored; 0 measured; 0 filtered out; finished in 143.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:25
