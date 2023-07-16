plain
...................................................ii...............i............................... 9100/11965
.................................................................................................... 9200/11965
.................................................................................................... 9300/11965
.................................................................................................... 9400/11965
....................................F.......F....................................................... 9500/11965
.................................................................................................... 9700/11965
.................i.i.......i........................................................................ 9800/11965
.........................................................................iiiiii.i..iiiiii.i......... 9900/11965
.................................................................................................... 10000/11965
---
---- [ui] ui/attributes/nonterminal-expansion.rs stdout ----
diff of stderr:

9    |
10    = note: this error originates in the macro `pass_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0589]: invalid `repr(align)` attribute
+    |
+    |
+ LL | pass_nonterminal!(n!());
+    |                   ^ not a non-negative number
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error: aborting due to 2 previous errors
+ 
+ For more information about this error, try `rustc --explain E0589`.
14 
---
To only update this specific test, also pass `--test-args attributes/nonterminal-expansion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/nonterminal-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/nonterminal-expansion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/nonterminal-expansion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected unsuffixed literal or identifier, found `n!()`
   |
   |
LL |         #[repr(align($n))] //~ ERROR expected unsuffixed literal or identifier, found `n!()`
...
...
LL | pass_nonterminal!(n!());
   |
   |
   = note: this error originates in the macro `pass_nonterminal` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0589]: invalid `repr(align)` attribute
   |
   |
LL | pass_nonterminal!(n!());
   |                   ^ not a non-negative number
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0589`.


------------------------------------------


---- [ui] ui/repr/repr-align-assign.rs stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/repr-align-assign.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-align-assign/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-align-assign/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/repr-align-assign.fixed:5:8
   |
LL | #[repr(align=8)] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(8))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^

error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/repr-align-assign.fixed:9:8
   |
LL | #[repr(align="8")] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(8))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^

error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/repr-align-assign.fixed:5:8
   |
LL | #[repr(align=8)] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(8))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^

error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/repr-align-assign.fixed:9:8
   |
LL | #[repr(align="8")] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(8))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0693`.


------------------------------------------


---- [ui] ui/repr/rustfix.rs stdout ----

error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/rustfix.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/rustfix/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/rustfix/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/rustfix.fixed:2:8
   |
LL | #[repr(align = r#"1"#)] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(1))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^

error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/rustfix.fixed:6:8
   |
LL | #[repr(align = r###"foo"###)] //~ ERROR incorrect `repr(align)` attribute format
   |                |
   |                must be a non-negative number
   |
help: use parentheses instead
help: use parentheses instead
   |
LL | #[repr(align(foo))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^   ^

error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/rustfix.fixed:10:8
   |
LL | #[repr(align = 1)] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(1))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^

error[E0589]: invalid `repr(align)` attribute
  --> /checkout/src/test/ui/repr/rustfix.fixed:22:14
   |
LL | #[repr(align("1"))] //~ ERROR invalid `repr(align)` attribute
   |
help: remove the quotes
   |
   |
LL | #[repr(align(1))] //~ ERROR invalid `repr(align)` attribute


error[E0589]: invalid `repr(align)` attribute
  --> /checkout/src/test/ui/repr/rustfix.fixed:26:14
   |
LL | #[repr(align(r############"1"############))] //~ ERROR invalid `repr(align)` attribute
   |
help: remove the quotes
   |
   |
LL | #[repr(align(1))] //~ ERROR invalid `repr(align)` attribute


error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/rustfix.fixed:2:8
   |
LL | #[repr(align = r#"1"#)] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(1))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^

error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/rustfix.fixed:6:8
   |
LL | #[repr(align = r###"foo"###)] //~ ERROR incorrect `repr(align)` attribute format
   |                |
   |                must be a non-negative number
   |
help: use parentheses instead
help: use parentheses instead
   |
LL | #[repr(align(foo))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^   ^

error[E0693]: incorrect `repr(align)` attribute format
  --> /checkout/src/test/ui/repr/rustfix.fixed:10:8
   |
LL | #[repr(align = 1)] //~ ERROR incorrect `repr(align)` attribute format
   |
help: use parentheses instead
   |
   |
LL | #[repr(align(1))] //~ ERROR incorrect `repr(align)` attribute format
   |             ^ ^

error[E0589]: invalid `repr(align)` attribute
  --> /checkout/src/test/ui/repr/rustfix.fixed:22:14
   |
LL | #[repr(align("1"))] //~ ERROR invalid `repr(align)` attribute
   |
help: remove the quotes
   |
   |
LL | #[repr(align(1))] //~ ERROR invalid `repr(align)` attribute


error[E0589]: invalid `repr(align)` attribute
  --> /checkout/src/test/ui/repr/rustfix.fixed:26:14
   |
LL | #[repr(align(r############"1"############))] //~ ERROR invalid `repr(align)` attribute
   |
help: remove the quotes
   |
   |
LL | #[repr(align(1))] //~ ERROR invalid `repr(align)` attribute

error: aborting due to 10 previous errors

Some errors have detailed explanations: E0589, E0693.
---
test result: FAILED. 11861 passed; 3 failed; 101 ignored; 0 measured; 0 filtered out; finished in 117.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:55
