plain
.................................................................................................... 400/12462
.................................................................................................... 500/12462
.................................................................................................... 600/12462
.................................................................................................... 700/12462
..i...F.......................................................................i..................... 800/12462
.................................................................................................... 1000/12462
.................................................................................................... 1100/12462
.................................................................................................... 1200/12462
.....i.............................................................................................. 1300/12462
---

---- [ui] ui/async-await/unresolved_type_param.rs stdout ----
diff of stderr:

34 LL |     bar().await;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
36 
36 
- error[E0698]: type inside `async fn` body must be known in this context
-   --> $DIR/unresolved_type_param.rs:9:5
-    |
- LL |     bar().await;
-    |     ^^^ cannot infer type for type parameter `T` declared on the function `bar`
-    |
- note: the type is part of the `async fn` body because of this `await`
-   --> $DIR/unresolved_type_param.rs:9:5
-    |
- LL |     bar().await;
- 
- 
- error[E0698]: type inside `async fn` body must be known in this context
-   --> $DIR/unresolved_type_param.rs:9:5
-    |
- LL |     bar().await;
-    |     ^^^ cannot infer type for type parameter `T` declared on the function `bar`
-    |
- note: the type is part of the `async fn` body because of this `await`
-   --> $DIR/unresolved_type_param.rs:9:5
-    |
- LL |     bar().await;
- 
- error: aborting due to 5 previous errors
+ error: aborting due to 3 previous errors
62 
62 
63 For more information about this error, try `rustc --explain E0698`.
64 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param/unresolved_type_param.stderr
To only update this specific test, also pass `--test-args async-await/unresolved_type_param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unresolved_type_param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0698]: type inside `async fn` body must be known in this context
   |
   |
LL |     bar().await;
   |     ^^^ cannot infer type for type parameter `T` declared on the function `bar`
   |
note: the type is part of the `async fn` body because of this `await`
   |
   |
LL |     bar().await;


error[E0698]: type inside `async fn` body must be known in this context
   |
   |
LL |     bar().await;
   |     ^^^ cannot infer type for type parameter `T` declared on the function `bar`
   |
note: the type is part of the `async fn` body because of this `await`
   |
   |
LL |     bar().await;


error[E0698]: type inside `async fn` body must be known in this context
   |
   |
LL |     bar().await;
   |     ^^^ cannot infer type for type parameter `T` declared on the function `bar`
   |
note: the type is part of the `async fn` body because of this `await`
   |
   |
LL |     bar().await;

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0698`.
---
test result: FAILED. 12343 passed; 1 failed; 118 ignored; 0 measured; 0 filtered out; finished in 148.61s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:00
