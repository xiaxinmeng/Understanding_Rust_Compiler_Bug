plain
.................................................................................................... 6000/11738
.........................................................i.......................................... 6100/11738
.................................................................................................... 6200/11738
...........................................................................................ii.ii.... 6300/11738
...i...i.............................................................................F...F.......... 6400/11738
.................................................................................................... 6600/11738
......................i............................................................................. 6700/11738
.................................................................................................... 6800/11738
.......................ii..............................................i............................ 6900/11738
---
.................................................................................................... 9400/11738
.................................................................................................... 9500/11738
..............................................................................i......i.............. 9600/11738
.................................................................................................... 9700/11738
........................iiiiiii..iiiiii.i........................................................... 9800/11738
.................................................................................................... 10000/11738
.................................................................................................... 10100/11738
.................................................................................................... 10200/11738
.................................................................................................... 10300/11738
---
.............................i.i.................................................................... 11700/11738
......................................
failures:

---- [ui] ui/lifetimes/issue-83753-invalid-associated-type-supertrait-hrtb.rs stdout ----

1 error[E0229]: associated type bindings are not allowed here
-   --> $DIR/issue-83753-invalid-associated-type-supertrait-hrtb.rs:3:21
+   --> $DIR/issue-83753-invalid-associated-type-supertrait-hrtb.rs:5:21
+   --> $DIR/issue-83753-invalid-associated-type-supertrait-hrtb.rs:5:21
3    |
4 LL |     fn bar(foo: Foo<Target = usize>) {}
5    |                     ^^^^^^^^^^^^^^ associated type not allowed here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-83753-invalid-associated-type-supertrait-hrtb/issue-83753-invalid-associated-type-supertrait-hrtb.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-83753-invalid-associated-type-supertrait-hrtb.rs`
error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-83753-invalid-associated-type-supertrait-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-83753-invalid-associated-type-supertrait-hrtb" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-83753-invalid-associated-type-supertrait-hrtb/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0229]: associated type bindings are not allowed here
  --> /checkout/src/test/ui/lifetimes/issue-83753-invalid-associated-type-supertrait-hrtb.rs:5:21
   |
LL |     fn bar(foo: Foo<Target = usize>) {}
   |                     ^^^^^^^^^^^^^^ associated type not allowed here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0229`.


------------------------------------------


---- [ui] ui/lifetimes/issue-83907-invalid-fn-like-path.rs stdout ----


1 error: free static item without body
-   --> $DIR/issue-83907-invalid-fn-like-path.rs:1:1
+   --> $DIR/issue-83907-invalid-fn-like-path.rs:3:1
3    |
4 LL | static STATIC_VAR_FIVE: &One();


7    |                               help: provide a definition for the static: `= <expr>;`
8 
9 error[E0412]: cannot find type `One` in this scope
-   --> $DIR/issue-83907-invalid-fn-like-path.rs:1:26
+   --> $DIR/issue-83907-invalid-fn-like-path.rs:3:26
11    |
12 LL | static STATIC_VAR_FIVE: &One();


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-83907-invalid-fn-like-path/issue-83907-invalid-fn-like-path.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-83907-invalid-fn-like-path.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-83907-invalid-fn-like-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-83907-invalid-fn-like-path" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-83907-invalid-fn-like-path/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: free static item without body
  --> /checkout/src/test/ui/lifetimes/issue-83907-invalid-fn-like-path.rs:3:1
   |
LL | static STATIC_VAR_FIVE: &One();
   |                               |
   |                               |
   |                               help: provide a definition for the static: `= <expr>;`

error[E0412]: cannot find type `One` in this scope
  --> /checkout/src/test/ui/lifetimes/issue-83907-invalid-fn-like-path.rs:3:26
   |
LL | static STATIC_VAR_FIVE: &One();

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0412`.
---
test result: FAILED. 11640 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 142.15s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:07
