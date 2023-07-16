plain
.................................................................................................... 9500/11870
.................................................................................................... 9600/11870
.......................................................................i......i..................... 9700/11870
.................................................................................................... 9800/11870
................iiiiiii..iiiiii.i................................................................... 9900/11870
.................................................................................................... 10100/11870
.................................................................................................... 10200/11870
.................................................................................................... 10300/11870
.................................................................................................... 10400/11870
.................................................................................................... 10400/11870
.................................................................................................... 10500/11870
.................................................................................................... 10600/11870
.i.F.F..................................i........................................................... 10700/11870
.................................................................................................... 10900/11870
.................................................................................................... 11000/11870
.................................................................................................... 11100/11870
....................................................................ii.............................. 11200/11870
---

---- [ui] ui/symbol-names/trait-objects.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN136_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u20$as$u20$trait_objects..Bar$GT$6method17h1e14a5f2d365272fE)
+ error: symbol-name(_ZN136_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u20$as$u20$trait_objects..Bar$GT$6method17hb7c7b07d18c7101dE)
3    |
4 LL |     #[rustc_symbol_name]

5    |     ^^^^^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = () as trait_objects::Bar>::method::h1e14a5f2d365272f)
+ error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = () as trait_objects::Bar>::method::hb7c7b07d18c7101d)
9    |
10 LL |     #[rustc_symbol_name]

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
16 LL |     #[rustc_symbol_name]
17    |     ^^^^^^^^^^^^^^^^^^^^
18 
- error: symbol-name(_ZN159_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u2b$core..marker..Send$u20$as$u20$trait_objects..Foo$GT$6method17he7a07961c9aaa367E)
+ error: symbol-name(_ZN159_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u2b$core..marker..Send$u20$as$u20$trait_objects..Foo$GT$6method17h9bab1ca2f4d270d0E)
21    |
22 LL |     #[rustc_symbol_name]

23    |     ^^^^^^^^^^^^^^^^^^^^
23    |     ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Foo>::method::he7a07961c9aaa367)
+ error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Foo>::method::h9bab1ca2f4d270d0)
27    |
28 LL |     #[rustc_symbol_name]

34 LL |     #[rustc_symbol_name]
34 LL |     #[rustc_symbol_name]
35    |     ^^^^^^^^^^^^^^^^^^^^
36 
- error: symbol-name(_ZN159_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u2b$core..marker..Send$u20$as$u20$trait_objects..Baz$GT$6method17ha53e6f99bf033f0bE)
+ error: symbol-name(_ZN159_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u2b$core..marker..Send$u20$as$u20$trait_objects..Baz$GT$6method17hd42e32faddabd832E)
39    |
40 LL |     #[rustc_symbol_name]

41    |     ^^^^^^^^^^^^^^^^^^^^
41    |     ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Baz>::method::ha53e6f99bf033f0b)
+ error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Baz>::method::hd42e32faddabd832)
45    |
46 LL |     #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.legacy/trait-objects.legacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/trait-objects.rs`

error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN136_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u20$as$u20$trait_objects..Bar$GT$6method17hb7c7b07d18c7101dE)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = () as trait_objects::Bar>::method::hb7c7b07d18c7101d)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn core::ops::function::FnMut<(&u8,)>+Output = () as trait_objects::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_ZN159_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u2b$core..marker..Send$u20$as$u20$trait_objects..Foo$GT$6method17h9bab1ca2f4d270d0E)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Foo>::method::h9bab1ca2f4d270d0)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_ZN159_$LT$$RF$dyn$u20$core..ops..function..FnMut$LT$$LP$$RF$u8$C$$RP$$GT$$u2b$Output$u20$$u3d$$u20$$LP$$RP$$u2b$core..marker..Send$u20$as$u20$trait_objects..Baz$GT$6method17hd42e32faddabd832E)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Baz>::method::hd42e32faddabd832)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn core::ops::function::FnMut<(&u8,)>+Output = ()+core::marker::Send as trait_objects::Baz>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 9 previous errors


------------------------------------------


---- [ui] ui/symbol-names/trait-objects.rs#v0 stdout ----


- error: symbol-name(_RNvXCs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCs54lBhuwykzk_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
+ error: symbol-name(_RNvXCs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCslvBEwyXmabl_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
3    |
4 LL |     #[rustc_symbol_name]

5    |     ^^^^^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<&dyn for<'a> core[3b0e14d6e1ad42d0]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[17891616a171812d]::Bar>::method)
+ error: demangling(<&dyn for<'a> core[fa89b33ef65358a9]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[17891616a171812d]::Bar>::method)
9    |
10 LL |     #[rustc_symbol_name]

16 LL |     #[rustc_symbol_name]
16 LL |     #[rustc_symbol_name]
17    |     ^^^^^^^^^^^^^^^^^^^^
18 
- error: symbol-name(_RNvXs_Cs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCs54lBhuwykzk_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
+ error: symbol-name(_RNvXs_Cs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCslvBEwyXmabl_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
21    |
22 LL |     #[rustc_symbol_name]

23    |     ^^^^^^^^^^^^^^^^^^^^
23    |     ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<&dyn for<'a> core[3b0e14d6e1ad42d0]::ops::function::FnMut<(&'a u8,), Output = ()> + core[3b0e14d6e1ad42d0]::marker::Send as trait_objects[17891616a171812d]::Foo>::method)
+ error: demangling(<&dyn for<'a> core[fa89b33ef65358a9]::ops::function::FnMut<(&'a u8,), Output = ()> + core[fa89b33ef65358a9]::marker::Send as trait_objects[17891616a171812d]::Foo>::method)
27    |
28 LL |     #[rustc_symbol_name]

34 LL |     #[rustc_symbol_name]
34 LL |     #[rustc_symbol_name]
35    |     ^^^^^^^^^^^^^^^^^^^^
36 
- error: symbol-name(_RNvXs0_Cs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCs54lBhuwykzk_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
+ error: symbol-name(_RNvXs0_Cs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCslvBEwyXmabl_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
39    |
40 LL |     #[rustc_symbol_name]

41    |     ^^^^^^^^^^^^^^^^^^^^
41    |     ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<&dyn for<'a> core[3b0e14d6e1ad42d0]::ops::function::FnMut<(&'a u8,), Output = ()> + core[3b0e14d6e1ad42d0]::marker::Send as trait_objects[17891616a171812d]::Baz>::method)
+ error: demangling(<&dyn for<'a> core[fa89b33ef65358a9]::ops::function::FnMut<(&'a u8,), Output = ()> + core[fa89b33ef65358a9]::marker::Send as trait_objects[17891616a171812d]::Baz>::method)
45    |
46 LL |     #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0/trait-objects.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/trait-objects.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvXCs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCslvBEwyXmabl_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[fa89b33ef65358a9]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[17891616a171812d]::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs_Cs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCslvBEwyXmabl_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[fa89b33ef65358a9]::ops::function::FnMut<(&'a u8,), Output = ()> + core[fa89b33ef65358a9]::marker::Send as trait_objects[17891616a171812d]::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs0_Cs21hi0yVfW1J_13trait_objectsRDG_INtNtNtCslvBEwyXmabl_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[fa89b33ef65358a9]::ops::function::FnMut<(&'a u8,), Output = ()> + core[fa89b33ef65358a9]::marker::Send as trait_objects[17891616a171812d]::Baz>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Baz>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^

---
test result: FAILED. 11772 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 102.09s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:34
