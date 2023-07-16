plain
.................................................................................................... 9300/12025
.................................................................................................... 9400/12025
.................................................................................................... 9500/12025
.................................................................................................... 9600/12025
...................................F......F..F...F.................................................. 9700/12025
.........................................................................................i.i......i. 9800/12025
.............iiiii.ii..iiiiii.i..................................................................... 10000/12025
.................................................................................................... 10100/12025
.................................................................................................... 10200/12025
.................................................................................................... 10300/12025
---
................i.i................................................................................. 12000/12025
.........................
failures:

---- [ui] ui/rfcs/rfc-2005-default-binding-mode/enum-fail.rs stdout ----


1 error[E0594]: cannot assign to `*x` which is behind a `&` reference
-   --> $DIR/enum.rs:9:5
+   --> $DIR/enum-fail.rs:9:5
3    |
4 LL |     *x += 1;
5    |     ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
6 
6 
7 error[E0594]: cannot assign to `*x` which is behind a `&` reference
-   --> $DIR/enum.rs:13:9
+   --> $DIR/enum-fail.rs:13:9
9    |
10 LL |         *x += 1;
11    |         ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
12 
12 
13 error[E0594]: cannot assign to `*x` which is behind a `&` reference
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/enum.rs:19:9
+   --> $DIR/enum-fail.rs:19:9
15    |
16 LL |         *x += 1;
17    |         ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/enum-fail/enum-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2005-default-binding-mode/enum-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/enum-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/enum-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/enum-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0594]: cannot assign to `*x` which is behind a `&` reference
  --> /checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/enum-fail.rs:9:5
   |
LL |     *x += 1; //~ ERROR cannot assign to `*x` which is behind a `&` reference
   |     ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written

error[E0594]: cannot assign to `*x` which is behind a `&` reference
  --> /checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/enum-fail.rs:13:9
   |
LL |         *x += 1; //~ ERROR cannot assign to `*x` which is behind a `&` reference
   |         ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written

error[E0594]: cannot assign to `*x` which is behind a `&` reference
  --> /checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/enum-fail.rs:19:9
   |
LL |         *x += 1; //~ ERROR cannot assign to `*x` which is behind a `&` reference
   |         ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0594`.


------------------------------------------


---- [ui] ui/rfcs/rfc-2005-default-binding-mode/for-fail.rs stdout ----

1 error[E0507]: cannot move out of a shared reference
-   --> $DIR/for.rs:6:23
+   --> $DIR/for-fail.rs:6:23
+   --> $DIR/for-fail.rs:6:23
3    |
4 LL |     for (n, mut m) in &tups {


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/for-fail/for-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2005-default-binding-mode/for-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/for-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/for-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/for-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0507]: cannot move out of a shared reference
  --> /checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/for-fail.rs:6:23
   |
LL |     for (n, mut m) in &tups {
   |             |
   |             data moved here
   |             data moved here
   |             move occurs because `m` has type `Foo`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.

---
3    |
4 LL |     match &s {
5    |           -- this expression has type `&&str`

10            found reference `&'static str`
12 error[E0308]: mismatched types
-   --> $DIR/lit.rs:16:9
+   --> $DIR/lit-fail.rs:16:9
14    |
14    |
15 LL |     match &s {
16    |           -- this expression has type `&&[u8]`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/lit-fail/lit-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2005-default-binding-mode/lit-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/lit-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/lit-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/lit-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/lit-fail.rs:7:13
   |
LL |     match &s {
   |           -- this expression has type `&&str`
LL |             "abc" => true, //~ ERROR mismatched types
   |             ^^^^^ expected `&str`, found `str`
   |
   = note:   expected type `&&str`
           found reference `&'static str`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/lit-fail.rs:16:9
   |
LL |     match &s {
LL |     match &s {
   |           -- this expression has type `&&[u8]`
LL |         b"abc" => true, //~ ERROR mismatched types
   |         ^^^^^^ expected `&[u8]`, found array `[u8; 3]`
   |
   = note:   expected type `&&[u8]`
           found reference `&'static [u8; 3]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/rfcs/rfc-2005-default-binding-mode/slice-non-exhaustive.rs stdout ----
diff of stderr:

1 error[E0004]: non-exhaustive patterns: `&[]` not covered
-   --> $DIR/slice.rs:4:11
+   --> $DIR/slice-non-exhaustive.rs:4:11
4 LL |     match sl {
4 LL |     match sl {
5    |           ^^ pattern `&[]` not covered

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/slice-non-exhaustive/slice-non-exhaustive.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2005-default-binding-mode/slice-non-exhaustive.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2005-default-binding-mode/slice-non-exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/slice-non-exhaustive" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2005-default-binding-mode/slice-non-exhaustive/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0004]: non-exhaustive patterns: `&[]` not covered
   |
   |
LL |     match sl { //~ ERROR non-exhaustive patterns
   |           ^^ pattern `&[]` not covered
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `&[u8]`

error: aborting due to previous error
---
test result: FAILED. 11924 passed; 4 failed; 97 ignored; 0 measured; 0 filtered out; finished in 124.79s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:35
