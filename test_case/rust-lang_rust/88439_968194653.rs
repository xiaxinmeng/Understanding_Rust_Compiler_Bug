plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12397 tests
.................................................................................................... 100/12397
........................................iiiiiiiiiiii...........i.i.................i.F.i............ 200/12397
.................................................................................................... 400/12397
.................................................................................................... 500/12397
.................................................................................................... 600/12397
......................................................................................i............. 700/12397
---

45    |                    |             clobber_abi
46    |                    generic outputs
47 
- error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
+ error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nomem`
50    |
50    |
51 LL | global_asm!("", options(nomem));

-    |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
+    |                         ^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
53 
- error: expected one of `)`, `att_syntax`, or `raw`, found `readonly`
+ error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `readonly`
56    |
56    |
57 LL | global_asm!("", options(readonly));

-    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
+    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
59 
- error: expected one of `)`, `att_syntax`, or `raw`, found `noreturn`
+ error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `noreturn`
62    |
62    |
63 LL | global_asm!("", options(noreturn));

-    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
+    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
65 
- error: expected one of `)`, `att_syntax`, or `raw`, found `pure`
+ error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `pure`
68    |
68    |
69 LL | global_asm!("", options(pure));

-    |                         ^^^^ expected one of `)`, `att_syntax`, or `raw`
+    |                         ^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
71 
- error: expected one of `)`, `att_syntax`, or `raw`, found `nostack`
+ error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nostack`
74    |
74    |
75 LL | global_asm!("", options(nostack));

-    |                         ^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
+    |                         ^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
77 
- error: expected one of `)`, `att_syntax`, or `raw`, found `preserves_flags`
+ error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `preserves_flags`
80    |
80    |
81 LL | global_asm!("", options(preserves_flags));

-    |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
+    |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
83 
84 error: invalid ABI for `clobber_abi`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-options/bad-options.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-options/bad-options.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/x86_64/bad-options.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/bad-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-options/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `nomem` and `readonly` options are mutually exclusive
   |
   |
LL |         asm!("", options(nomem, readonly));


error: the `pure` and `noreturn` options are mutually exclusive
   |
   |
LL |         asm!("", options(pure, nomem, noreturn));


error: asm with the `pure` option must have at least one output
   |
   |
LL |         asm!("", options(pure, nomem, noreturn));


error: asm with the `pure` option must have at least one output
   |
   |
LL |         asm!("{}", in(reg) foo, options(pure, nomem));


error: asm outputs are not allowed with the `noreturn` option
   |
   |
LL |         asm!("{}", out(reg) foo, options(noreturn));


error: asm with `clobber_abi` must specify explicit registers for outputs
   |
   |
LL |         asm!("{}", out(reg) foo, clobber_abi("C"));
   |                    ^^^^^^^^^^^^  ---------------- clobber_abi
   |                    generic outputs


error: asm with `clobber_abi` must specify explicit registers for outputs
   |
   |
LL |         asm!("{}", out(reg) foo, clobber_abi("C"), clobber_abi("C"));
   |                    ^^^^^^^^^^^^  ----------------  ---------------- clobber_abi
   |                    |             clobber_abi
   |                    generic outputs


error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem));
   |                         ^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`

error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `readonly`
   |
   |
LL | global_asm!("", options(readonly));
   |                         ^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`

error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `noreturn`
   |
   |
LL | global_asm!("", options(noreturn));
   |                         ^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`

error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `pure`
   |
   |
LL | global_asm!("", options(pure));
   |                         ^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`

error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nostack`
   |
   |
LL | global_asm!("", options(nostack));
   |                         ^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`

error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `preserves_flags`
   |
   |
LL | global_asm!("", options(preserves_flags));
   |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`

error: invalid ABI for `clobber_abi`
   |
   |
LL |         asm!("", clobber_abi("foo"));
   |
   |
   = note: the following ABIs are supported on this target: `C`, `system`, `efiapi`, `win64`, `sysv64`

error: `C` ABI specified multiple times
   |
   |
LL |         asm!("{}", out(reg) foo, clobber_abi("C"), clobber_abi("C"));
   |                                  ----------------  ^^^^^^^^^^^^^^^^
   |                                  previously specified here

error: aborting due to 15 previous errors

---
test result: FAILED. 12282 passed; 1 failed; 114 ignored; 0 measured; 0 filtered out; finished in 135.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:43
