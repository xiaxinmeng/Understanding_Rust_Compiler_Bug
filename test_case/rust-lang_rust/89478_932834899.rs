plain
---- [ui] ui/associated-types/impl-wf-cycle-1.rs stdout ----
diff of stderr:

15    |
16 LL | impl<T: Grault> Grault for (T,)
17    |                 ^^^^^^     ^^^^
-    = note: 1 redundant requirements hidden
+    = note: 1 redundant requirement hidden
19    = note: required because of the requirements on the impl of `Grault` for `(T,)`
20 
21 error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
29    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
30 LL | impl<T: Grault> Grault for (T,)
31    |                 ^^^^^^     ^^^^
-    = note: 1 redundant requirements hidden
+    = note: 1 redundant requirement hidden
33    = note: required because of the requirements on the impl of `Grault` for `(T,)`
34 
35 error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
43    |
43    |
44 LL | impl<T: Grault> Grault for (T,)
45    |                 ^^^^^^     ^^^^
-    = note: 1 redundant requirements hidden
+    = note: 1 redundant requirement hidden
47    = note: required because of the requirements on the impl of `Grault` for `(T,)`
49 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1/impl-wf-cycle-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/impl-wf-cycle-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
   |
LL | / impl<T: Grault> Grault for (T,)
LL | | where
LL | |     Self::A: Baz,
LL | |     Self::B: Fiz,
...  |
LL | |     //~^ ERROR overflow evaluating the requirement `<(T,) as Grault>::A == _`
LL | | }
   |
   |
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   |                 ^^^^^^     ^^^^
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Grault` for `(T,)`

error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
LL |     type A = ();
   |     ^^^^^^^^^^^^
   |
   |
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   |                 ^^^^^^     ^^^^
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Grault` for `(T,)`

error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
LL |     type B = bool;
   |     ^^^^^^^^^^^^^^
   |
   |
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   |                 ^^^^^^     ^^^^
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0275`.

---
test result: FAILED. 12123 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 134.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:33
