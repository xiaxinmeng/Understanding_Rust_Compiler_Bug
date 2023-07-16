plain

---- [ui] rustdoc-ui/intra-doc/non-path-primitives.rs stdout ----
diff of stderr:

53 LL | //! [fn::eq]
54    |      ^^^^^^ the builtin type `fn` has no associated item named `eq`
55 
- error: unresolved link to `never::eq`
-   --> $DIR/non-path-primitives.rs:31:6
-    |
- LL | //! [never::eq]
-    |      ^^^^^^^^^ the builtin type `never` has no associated item named `eq`
62 error: unresolved link to `reference::deref`
63   --> $DIR/non-path-primitives.rs:35:6
64    |


65 LL | //! [reference::deref]
66    |      ^^^^^^^^^^^^^^^^ the builtin type `reference` has no associated item named `deref`
- error: aborting due to 9 previous errors
+ error: aborting due to 8 previous errors
69 
70 
---
To only update this specific test, also pass `--test-args intra-doc/non-path-primitives.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/non-path-primitives" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/non-path-primitives/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `T`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:12:7
   |
LL | //! [[T]::rotate_left] //~ ERROR unresolved link to `T`
   |       ^ no item named `T` in scope
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Z`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:14:5
   |
   |
LL | //![Z]([T; N]::map) //~ ERROR unresolved link to `Z`
   |     ^ no item named `Z` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Z`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:17:6
   |
   |
LL | //! [Z][] //~ ERROR unresolved link to `Z`
   |      ^ no item named `Z` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Z`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:19:6
   |
   |
LL | //! [Z]: [T; N]::map //~ ERROR unresolved link to `Z`
   |      ^ no item named `Z` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `unit::eq`
   |
   |
LL | //! [unit::eq] //~ ERROR unresolved
   |      ^^^^^^^^ the builtin type `unit` has no associated item named `eq`

error: unresolved link to `tuple::eq`
   |
   |
LL | //! [tuple::eq] //~ ERROR unresolved
   |      ^^^^^^^^^ the builtin type `tuple` has no associated item named `eq`

error: unresolved link to `fn::eq`
   |
   |
LL | //! [fn::eq] //~ ERROR unresolved
   |      ^^^^^^ the builtin type `fn` has no associated item named `eq`
error: unresolved link to `reference::deref`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:35:6
   |
   |
LL | //! [reference::deref] //~ ERROR unresolved
   |      ^^^^^^^^^^^^^^^^ the builtin type `reference` has no associated item named `deref`
error: aborting due to 8 previous errors


------------------------------------------
---
test result: FAILED. 150 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.13s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:27:32
