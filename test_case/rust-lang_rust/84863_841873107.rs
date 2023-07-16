plain
.................................................................................................... 9500/11879
.................................................................................................... 9600/11879
...............................................................................i......i............. 9700/11879
.................................................................................................... 9800/11879
........................iiiiiii..iiiiii.i........................................................... 9900/11879
.................................................................................................... 10100/11879
.................................................................................................... 10200/11879
.................................................................................................... 10300/11879
.................................................................................................... 10400/11879
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 32 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.179 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.47s

 finished in 3.530 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 121 tests
..................................F.F..........................................F.........F.......... 100/121
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..............F......

---- [ui] rustdoc-ui/failed-doctest-compile-fail.rs stdout ----
diff of stdout:


11     $DIR/failed-doctest-compile-fail.rs - Foo (line 9)
13 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
+ 
14 

---
To only update this specific test, also pass `--test-args failed-doctest-compile-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-compile-fail" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-compile-fail/auxiliary"
------------------------------------------

running 1 test
running 1 test
test /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9) - compile fail ... FAILED
failures:

---- /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9) stdout ----
---- /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9) stdout ----
Test compiled successfully, but it's marked `compile_fail`.
failures:
    /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
---

---- [ui] rustdoc-ui/failed-doctest-missing-codes.rs stdout ----
diff of stdout:

22     $DIR/failed-doctest-missing-codes.rs - Foo (line 9)
24 test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
+ 
25 

---
To only update this specific test, also pass `--test-args failed-doctest-missing-codes.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-missing-codes/auxiliary"
------------------------------------------

running 1 test
running 1 test
test /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs - Foo (line 9) - compile fail ... FAILED
failures:

---- /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs - Foo (line 9) stdout ----
error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs:10:13
   |
LL | let x: () = 5i32;
   |        --   ^^^^ expected `()`, found `i32`
   |        expected due to this

error: aborting due to previous error


For more information about this error, try `rustc --explain E0308`.
Some expected error codes were not found: ["E0004"]
failures:
    /checkout/src/test/rustdoc-ui/failed-doctest-missing-codes.rs - Foo (line 9)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
---
To only update this specific test, also pass `--test-args issue-80992.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-80992.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-80992" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-80992/auxiliary"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc-ui/issue-80992.rs - test (line 7) - compile fail ... ok
---

---- [ui] rustdoc-ui/no-run-flag.rs stdout ----
diff of stdout:

9 test $DIR/no-run-flag.rs - f (line 8) - compile ... ok
11 test result: ok. 6 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in $TIME
+ 
12 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/no-run-flag/no-run-flag.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args no-run-flag.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/no-run-flag.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/no-run-flag" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--test" "--no-run" "--test-args=--test-threads=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/no-run-flag/auxiliary"
------------------------------------------

running 7 tests
running 7 tests
test /checkout/src/test/rustdoc-ui/no-run-flag.rs - f (line 11) - compile ... ok
test /checkout/src/test/rustdoc-ui/no-run-flag.rs - f (line 14) ... ignored
test /checkout/src/test/rustdoc-ui/no-run-flag.rs - f (line 17) - compile ... ok
test /checkout/src/test/rustdoc-ui/no-run-flag.rs - f (line 23) - compile fail ... ok
test /checkout/src/test/rustdoc-ui/no-run-flag.rs - f (line 28) - compile ... ok
test /checkout/src/test/rustdoc-ui/no-run-flag.rs - f (line 32) - compile ... ok
test /checkout/src/test/rustdoc-ui/no-run-flag.rs - f (line 8) - compile ... ok
test result: ok. 6 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.23s


------------------------------------------
---

---- [ui] rustdoc-ui/test-type.rs stdout ----
diff of stdout:

7 test $DIR/test-type.rs - f (line 9) ... ok
9 test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in $TIME
+ 
10 

---
To only update this specific test, also pass `--test-args test-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/test-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/test-type" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "--test-args=--test-threads=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/test-type/auxiliary"
------------------------------------------

running 5 tests
test /checkout/src/test/rustdoc-ui/test-type.rs - f (line 12) ... ignored
test /checkout/src/test/rustdoc-ui/test-type.rs - f (line 12) ... ignored
test /checkout/src/test/rustdoc-ui/test-type.rs - f (line 15) - compile ... ok
test /checkout/src/test/rustdoc-ui/test-type.rs - f (line 21) - compile fail ... ok
test /checkout/src/test/rustdoc-ui/test-type.rs - f (line 6) ... ok
test /checkout/src/test/rustdoc-ui/test-type.rs - f (line 9) ... ok
test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.47s


------------------------------------------
---
test result: FAILED. 116 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.66s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:29:11
