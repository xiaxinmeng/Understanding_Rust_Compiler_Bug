plain

---- [ui] ui/error-codes/E0308-2.rs stdout ----
diff of stderr:

4 LL | impl Eq for &dyn DynEq {}
5    |      ^^ lifetime mismatch
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: expected trait `PartialEq`
-               found trait `PartialEq`
-               found trait `PartialEq`
+    = note: expected trait `<&dyn DynEq as PartialEq>`
+               found trait `<&(dyn DynEq + 'static) as PartialEq>`
9 note: the lifetime `'_` as defined on the impl at 9:13...
10   --> $DIR/E0308-2.rs:9:13


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0308-2/E0308-2.stderr
To only update this specific test, also pass `--test-args error-codes/E0308-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0308-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0308-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0308-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/error-codes/E0308-2.rs:9:6
   |
LL | impl Eq for &dyn DynEq {} //~ ERROR E0308
   |      ^^ lifetime mismatch
   |
   = note: expected trait `<&dyn DynEq as PartialEq>`
              found trait `<&(dyn DynEq + 'static) as PartialEq>`
note: the lifetime `'_` as defined on the impl at 9:13...
   |
   |
LL | impl Eq for &dyn DynEq {} //~ ERROR E0308
   |             ^
   = note: ...does not necessarily outlive the static lifetime
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/issues/issue-20831-debruijn.rs stdout ----
diff of stderr:

19    |
20 LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
21    |        ^^^^^^^^^
-    = note: expected `Publisher<'_>`
-               found `Publisher<'_>`
+    = note: expected `<MyStruct<'a> as Publisher<'_>>`
+               found `<MyStruct<'_> as Publisher<'_>>`
25 error: aborting due to previous error
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn/issue-20831-debruijn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-20831-debruijn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20831-debruijn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
   |
   |
LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
   |
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined on the method body at 28:58...
   |
   |
LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
   |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...but the lifetime must also be valid for the lifetime `'a` as defined on the impl at 26:6...
   |
   |
LL | impl<'a> Publisher<'a> for MyStruct<'a> {
   |      ^^
note: ...so that the types are compatible
   |
   |
LL |     fn subscribe(&mut self, t : Box<dyn Subscriber<Input=<Self as Publisher>::Output> + 'a>) {
   |        ^^^^^^^^^
   = note: expected `<MyStruct<'a> as Publisher<'_>>`
              found `<MyStruct<'_> as Publisher<'_>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.


------------------------------------------


---- [ui] ui/issues/issue-65230.rs stdout ----

error: /checkout/src/test/ui/issues/issue-65230.rs:10: unexpected error: '10:28: 12:6: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements [E0495]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65230.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65230" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65230/auxiliary"
    Error {
        line_num: 10,
        kind: Some(
            Error,
            Error,
        ),
        msg: "10:28: 12:6: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements [E0495]",
]

thread '[ui] ui/issues/issue-65230.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1538:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
test result: FAILED. 12140 passed; 3 failed; 115 ignored; 0 measured; 0 filtered out; finished in 103.09s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:42
