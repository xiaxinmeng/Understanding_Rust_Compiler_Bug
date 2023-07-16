plain
---- [ui] ui/lifetimes/lifetime-errors/issue_74400.rs stdout ----
diff of stderr:

- error[E0308]: mismatched types
+ error: implementation of `FnOnce` is not general enough
3    |
3    |
4 LL |     is_sorted_and_has_duplicates_by(data, identity)
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime mismatch
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
6    |
-    = note: expected type `FnOnce<(&T,)>`
-               found type `FnOnce<(&T,)>`
- note: the lifetime requirement is introduced here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/issue_74400.rs:10:25
-    |
- LL |     key: impl Fn(&T) -> S,
-    |                         ^
+    = note: `fn(&'2 T) -> &'2 T {identity::<&'2 T>}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
+    = note: ...but it actually implements `FnOnce<(&'2 T,)>`, for some specific lifetime `'2`
15 error: aborting due to previous error
16 

- For more information about this error, try `rustc --explain E0308`.
- For more information about this error, try `rustc --explain E0308`.
18 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400/issue_74400.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/issue_74400.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/issue_74400.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: implementation of `FnOnce` is not general enough
   |
   |
LL |     is_sorted_and_has_duplicates_by(data, identity) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&'2 T) -> &'2 T {identity::<&'2 T>}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 T,)>`, for some specific lifetime `'2`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11970 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 131.82s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:27
