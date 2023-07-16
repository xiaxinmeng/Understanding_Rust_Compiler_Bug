plain
.................................................................................................... 10600/12296
.................................................................................................... 10700/12296
.................................................................................................... 10800/12296
.................................................................................................... 10900/12296
.......................................................ii...F...........................i........... 11000/12296
.................................................................................................... 11200/12296
.................................................................................................... 11300/12296
.................................................................................................... 11400/12296
.................................................................................................... 11500/12296
---
...................................................................................i.ii............. 12200/12296
................................................................................................
failures:

---- [ui] ui/symbol-names/trait-objects.rs#v0 stdout ----


1 error: symbol-name(_RNvXCs$CRATE_HASH_13trait_objectsRDG_INtNtNtCs$CRATE_HASH_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
-   --> $DIR/trait-objects.rs:16:5
3    |
4 LL |     #[rustc_symbol_name]
5    |     ^^^^^^^^^^^^^^^^^^^^


6 
7 error: demangling(<&dyn for<'a> core[$HASH]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[$HASH]::Bar>::method)
-   --> $DIR/trait-objects.rs:16:5
9    |
10 LL |     #[rustc_symbol_name]
11    |     ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects::Bar>::method)
-   --> $DIR/trait-objects.rs:16:5
15    |
16 LL |     #[rustc_symbol_name]
17    |     ^^^^^^^^^^^^^^^^^^^^


18 
19 error: symbol-name(_RNvXs_Cs$CRATE_HASH_13trait_objectsRDG_INtNtNtCs$CRATE_HASH_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
-   --> $DIR/trait-objects.rs:28:5
21    |
22 LL |     #[rustc_symbol_name]
23    |     ^^^^^^^^^^^^^^^^^^^^


24 
25 error: demangling(<&dyn for<'a> core[$HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[$HASH]::marker::Send as trait_objects[$HASH]::Foo>::method)
-   --> $DIR/trait-objects.rs:28:5
27    |
28 LL |     #[rustc_symbol_name]
29    |     ^^^^^^^^^^^^^^^^^^^^


30 
31 error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Foo>::method)
-   --> $DIR/trait-objects.rs:28:5
33    |
34 LL |     #[rustc_symbol_name]
35    |     ^^^^^^^^^^^^^^^^^^^^


36 
37 error: symbol-name(_RNvXs0_Cs$CRATE_HASH_13trait_objectsRDG_INtNtNtCs$CRATE_HASH_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
-   --> $DIR/trait-objects.rs:40:5
39    |
40 LL |     #[rustc_symbol_name]
41    |     ^^^^^^^^^^^^^^^^^^^^


42 
43 error: demangling(<&dyn for<'a> core[$HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[$HASH]::marker::Send as trait_objects[$HASH]::Baz>::method)
-   --> $DIR/trait-objects.rs:40:5
45    |
46 LL |     #[rustc_symbol_name]
47    |     ^^^^^^^^^^^^^^^^^^^^


48 
49 error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Baz>::method)
-   --> $DIR/trait-objects.rs:40:5
51    |
52 LL |     #[rustc_symbol_name]
53    |     ^^^^^^^^^^^^^^^^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0/trait-objects.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/trait-objects.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvXCsaRzhxLaKEof_13trait_objectsRDG_INtNtNtCsamUzZWRLrk4_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[78c7d8fc7dbd73a2]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[7e8a38e0f5495e91]::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs_CsaRzhxLaKEof_13trait_objectsRDG_INtNtNtCsamUzZWRLrk4_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[78c7d8fc7dbd73a2]::ops::function::FnMut<(&'a u8,), Output = ()> + core[78c7d8fc7dbd73a2]::marker::Send as trait_objects[7e8a38e0f5495e91]::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs0_CsaRzhxLaKEof_13trait_objectsRDG_INtNtNtCsamUzZWRLrk4_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[78c7d8fc7dbd73a2]::ops::function::FnMut<(&'a u8,), Output = ()> + core[78c7d8fc7dbd73a2]::marker::Send as trait_objects[7e8a38e0f5495e91]::Baz>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Baz>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:36
