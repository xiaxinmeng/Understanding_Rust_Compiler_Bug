plain
.....................................................................i.............................. 8300/11537
.................................................................................................... 8400/11537
.................................................................................................... 8500/11537
.................................................................................................... 8600/11537
............................................................................F................F...... 8700/11537
..........................i...............i......................................................... 8900/11537
.................................................................................................... 9000/11537
.................................................................................................... 9100/11537
.................................................................................................... 9200/11537
.................................................................................................... 9200/11537
.................................................................................................... 9300/11537
.................................................................................................... 9400/11537
......................................................................i......i...................... 9500/11537
.................................................................................................... 9600/11537
................iiiiiii..iiiiii..i.................................................................. 9700/11537
.................................................................................................... 9900/11537
.................................................................................................... 10000/11537
.................................................................................................... 10100/11537
.................................................................................................... 10200/11537
---

---- [ui] ui/error-codes/E0451.rs stdout ----
diff of stderr:

- error[E0451]: field `b` of struct `Foo` is private
+ error[E0603]: cannot match on a field named `b` of struct `Foo`, which is not accessible in current scope
2   --> $DIR/E0451.rs:14:21
3    |
4 LL |     let bar::Foo{a, b} = foo;
-    |                     ^ private field
+    |                     ^
6 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error[E0451]: field `b` of struct `Foo` is private
-   --> $DIR/E0451.rs:18:29
-    |
- LL |     let f = bar::Foo{ a: 0, b: 0 };
+ error: aborting due to previous error
12 
- error: aborting due to 2 previous errors
- 
- 
- For more information about this error, try `rustc --explain E0451`.
+ For more information about this error, try `rustc --explain E0603`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0451/E0451.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0451.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0451.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0451" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0451/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: cannot match on a field named `b` of struct `Foo`, which is not accessible in current scope
   |
   |
LL |     let bar::Foo{a, b} = foo; //~ ERROR E0451

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
For more information about this error, try `rustc --explain E0603`.

------------------------------------------


---- [ui] ui/privacy/private-struct-field-pattern.rs stdout ----
diff of stderr:

- error[E0451]: field `x` of struct `Foo` is private
+ error[E0603]: cannot match on a field named `x` of struct `Foo`, which is not accessible in current scope
3    |
3    |
4 LL |         Foo { x: _ } => {}
-    |               ^^^^ private field
+    |               ^
6 
7 error: aborting due to previous error
---
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field-pattern/private-struct-field-pattern.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/private-struct-field-pattern.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-struct-field-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field-pattern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-struct-field-pattern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: cannot match on a field named `x` of struct `Foo`, which is not accessible in current scope
   |
   |
LL |         Foo { x: _ } => {}  //~ ERROR field `x` of struct `Foo` is private

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
For more information about this error, try `rustc --explain E0603`.

------------------------------------------


---- [ui] ui/privacy/union-field-privacy-1.rs stdout ----
diff of stderr:

- error[E0451]: field `c` of union `U` is private
-   --> $DIR/union-field-privacy-1.rs:12:20
-    |
- LL |     let u = m::U { c: 0 };
- 
- 
- error[E0451]: field `c` of union `U` is private
+ error[E0603]: cannot match on a field named `c` of union `U`, which is not accessible in current scope
9    |
9    |
10 LL |     let m::U { c } = u;
-    |                ^ private field
+    |                ^
12 
- error: aborting due to 2 previous errors
---
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/union-field-privacy-1/union-field-privacy-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/union-field-privacy-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/union-field-privacy-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/union-field-privacy-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/union-field-privacy-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0603]: cannot match on a field named `c` of union `U`, which is not accessible in current scope
   |
   |
LL |     let m::U { c } = u; //~ ERROR field `c` of union `U` is private

error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
---
test result: FAILED. 11441 passed; 3 failed; 93 ignored; 0 measured; 0 filtered out; finished in 131.70s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:32
