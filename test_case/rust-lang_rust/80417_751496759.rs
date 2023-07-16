plain
.................................................................................................... 8900/11103
.................................................................................................... 9000/11103
.................................................................................................... 9100/11103
..........i......................................................................................... 9200/11103
.........................................iiiiii..iiiiii.i........................................... 9300/11103
...................................................................................................i 9400/11103
.................................................................................................... 9600/11103
.................................................................................................... 9700/11103
.................................................................................................... 9800/11103
.................................................................................................... 9900/11103
---
diff of stderr:

11    | |_^
12    |
13    = note: required because of the requirements on the impl of `Grault` for `(T,)`
-    = note: 1 redundant requirements hidden
15    = note: required because of the requirements on the impl of `Grault` for `(T,)`
16 
17 error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
21    |     ^^^^^^^^^^^^
22    |
22    |
23    = note: required because of the requirements on the impl of `Grault` for `(T,)`
-    = note: 1 redundant requirements hidden
25    = note: required because of the requirements on the impl of `Grault` for `(T,)`
26 
27 error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
31    |     ^^^^^^^^^^^^^^
32    |
32    |
33    = note: required because of the requirements on the impl of `Grault` for `(T,)`
-    = note: 1 redundant requirements hidden
35    = note: required because of the requirements on the impl of `Grault` for `(T,)`
37 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1/impl-wf-cycle-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/impl-wf-cycle-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1/auxiliary"
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
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
   = note: required because of the requirements on the impl of `Grault` for `(T,)`

error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
LL |     type A = ();
   |     ^^^^^^^^^^^^
   |
   |
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
   = note: required because of the requirements on the impl of `Grault` for `(T,)`

error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
   |
LL |     type B = bool;
   |     ^^^^^^^^^^^^^^
   |
   |
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0275`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpuasmprinter amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrasmprinter avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitwriter bpf bpfasmparser bpfasmprinter bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader lanai lanaiasmparser lanaiasmprinter lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes perfjitevents powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:41
