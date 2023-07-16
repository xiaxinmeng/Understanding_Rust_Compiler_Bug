plain

---- [ui] ui/async-await/issue-61076.rs stdout ----
diff of stderr:

78    |                     ^^^^^ checked the `Output` of this `async fn`, expected opaque type
79    = note: expected opaque type `impl Future`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
80                    found struct `Tuple`
- help: consider `await`ing on the `Future`
-    |
- LL |     match tuple().await {
85 
86 error: aborting due to 6 previous errors
87 

---
To only update this specific test, also pass `--test-args async-await/issue-61076.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the `?` operator can only be applied to values that implement `Try`
  --> /checkout/src/test/ui/async-await/issue-61076.rs:42:5
   |
LL |     foo()?; //~ ERROR the `?` operator can only be applied to values that implement `Try`
   |     ^^^^^^ the `?` operator cannot be applied to type `impl Future`
   |
   = help: the trait `Try` is not implemented for `impl Future`
note: required by `branch`
   |
   |
LL |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
   |
LL |     foo().await?; //~ ERROR the `?` operator can only be applied to values that implement `Try`

error[E0277]: the `?` operator can only be applied to values that implement `Try`
  --> /checkout/src/test/ui/async-await/issue-61076.rs:67:5
   |
   |
LL |     t?; //~ ERROR the `?` operator can only be applied to values that implement `Try`
   |     ^^ the `?` operator cannot be applied to type `T`
   = help: the trait `Try` is not implemented for `T`
   = help: the trait `Try` is not implemented for `T`
note: required by `branch`
   |
   |
LL |     fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
   |
LL |     t.await?; //~ ERROR the `?` operator can only be applied to values that implement `Try`

error[E0609]: no field `0` on type `impl Future`
  --> /checkout/src/test/ui/async-await/issue-61076.rs:78:26
   |
   |
LL |     let _: i32 = tuple().0; //~ ERROR no field `0`
   |                          ^ field not available in `impl Future`, but it is available in its `Output`
   |
help: consider `await`ing on the `Future` and access the field of its `Output`
   |
LL |     let _: i32 = tuple().await.0; //~ ERROR no field `0`


error[E0609]: no field `a` on type `impl Future`
   |
   |
LL |     let _: i32 = struct_().a; //~ ERROR no field `a`
   |                            ^ field not available in `impl Future`, but it is available in its `Output`
   |
help: consider `await`ing on the `Future` and access the field of its `Output`
   |
LL |     let _: i32 = struct_().await.a; //~ ERROR no field `a`


error[E0599]: no method named `method` found for opaque type `impl Future` in the current scope
   |
   |
LL |     struct_().method(); //~ ERROR no method named
   |               ^^^^^^ method not found in `impl Future`
   |
help: consider `await`ing on the `Future` and calling the method on its `Output`
   |
LL |     struct_().await.method(); //~ ERROR no method named

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/issue-61076.rs:94:9
   |
   |
LL |         Tuple(_) => {} //~ ERROR mismatched types
   |         ^^^^^^^^ expected opaque type, found struct `Tuple`
   |
note: while checking the return type of the `async fn`
   |
   |
LL | async fn tuple() -> Tuple {
   |                     ^^^^^ checked the `Output` of this `async fn`, expected opaque type
   = note: expected opaque type `impl Future`
                   found struct `Tuple`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0277, E0308, E0599, E0609.
For more information about an error, try `rustc --explain E0277`.
---
test result: FAILED. 12283 passed; 1 failed; 111 ignored; 0 measured; 0 filtered out; finished in 143.19s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:52
