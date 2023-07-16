plain
---- [ui] ui/suggestions/crate-or-module-typo.rs stdout ----
diff of stderr:

3    |
4 LL | use my_cor::mem;
5    |     ^^^^^^ use of undeclared crate or module `my_cor`
+ help: there is a crate or module with a similar name
+    |
+    |
+ LL | use my_core::mem;
6 
6 
7 error[E0432]: unresolved import `my_core`


56    |         ^^^^^^ use of undeclared crate or module `my_cor`
57 ...
58 LL | a!();
+    | ---- in this macro invocation
60    |
60    |
61    = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)
62 help: there is a crate or module with a similar name

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/crate-or-module-typo/crate-or-module-typo.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/crate-or-module-typo.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/crate-or-module-typo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/crate-or-module-typo" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/crate-or-module-typo/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `my_cor`
   |
   |
LL | use my_cor::mem;
   |     ^^^^^^ use of undeclared crate or module `my_cor`
help: there is a crate or module with a similar name
   |
   |
LL | use my_core::mem;


error[E0432]: unresolved import `my_core`
   |
   |
LL | use my_core::mem;
   |     ^^^^^^^ use of undeclared crate or module `my_core`
error[E0432]: unresolved import `aa`
  --> /checkout/src/test/ui/suggestions/crate-or-module-typo.rs:35:5
   |
   |
LL | use aa::bar; //~ ERROR unresolved import `aa`
   |     ^^ use of undeclared crate or module `aa`
error[E0433]: failed to resolve: use of undeclared crate or module `st`
  --> /checkout/src/test/ui/suggestions/crate-or-module-typo.rs:39:5
   |
   |
LL | use st::cell::Cell; //~ ERROR failed to resolve: use of undeclared crate or module `st`
   |     ^^ use of undeclared crate or module `st`
help: there is a crate or module with a similar name
   |
   |
LL | use std::cell::Cell; //~ ERROR failed to resolve: use of undeclared crate or module `st`

error[E0432]: unresolved import `bb`
  --> /checkout/src/test/ui/suggestions/crate-or-module-typo.rs:37:5
   |
   |
LL | use bb::bar; //~ ERROR unresolved import `bb`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |     ^^ use of undeclared crate or module `bb`
help: there is a crate or module with a similar name
   |
   |
LL | use b::bar; //~ ERROR unresolved import `bb`

error[E0432]: unresolved import `fooo`
  --> /checkout/src/test/ui/suggestions/crate-or-module-typo.rs:41:5
   |
   |
LL | use fooo::bar; //~ ERROR unresolved import `fooo`
   |     ^^^^ use of undeclared crate or module `fooo`
help: there is a crate or module with a similar name
   |
   |
LL | use foo::bar; //~ ERROR unresolved import `fooo`


error[E0432]: unresolved import `my_cor`
   |
   |
LL |     use my_cor::mem;
   |         ^^^^^^ use of undeclared crate or module `my_cor`
...
LL | a!();
   |
   |
   = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)
help: there is a crate or module with a similar name
   |
LL |     use my_core::mem;

error[E0433]: failed to resolve: use of undeclared crate or module `fooo`
  --> /checkout/src/test/ui/suggestions/crate-or-module-typo.rs:22:20
   |
   |
LL |     pub fn bar() { fooo::baz(); } //~ ERROR failed to resolve: use of undeclared crate or module `fooo`
   |                    ^^^^ use of undeclared crate or module `fooo`
error[E0433]: failed to resolve: use of undeclared crate or module `st`
  --> /checkout/src/test/ui/suggestions/crate-or-module-typo.rs:44:10
   |
   |
LL |     bar: st::cell::Cell<bool> //~ ERROR failed to resolve: use of undeclared crate or module `st`
   |          ^^ use of undeclared crate or module `st`
help: there is a crate or module with a similar name
   |
   |
LL |     bar: std::cell::Cell<bool> //~ ERROR failed to resolve: use of undeclared crate or module `st`

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0432, E0433.
---
test result: FAILED. 12318 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 142.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:21
