plain
diff of stderr:

48    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
49    |
50 note: required because of the requirements on the impl of `Copy` for `Fooy<T>`
-   --> $DIR/impl_bounds.rs:12:10
52    |
53 LL | #[derive(Copy, Clone)]
54    |          ^^^^


73    |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
74    |
75 note: required because of the requirements on the impl of `Copy` for `Fooy<T>`
-   --> $DIR/impl_bounds.rs:12:10
77    |
78 LL | #[derive(Copy, Clone)]
79    |          ^^^^

---
To only update this specific test, also pass `--test-args generic-associated-types/impl_bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/impl_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/impl_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/impl_bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     type A<'a> where Self: 'static = (&'a ());
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'static`...
   = note: ...so that the type `Fooy<T>` will meet its required lifetime bounds

error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
   |
   |
note: lifetime parameter instantiated with the lifetime `'b` as defined on the associated item at 17:16
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
   |                ^^
note: but lifetime parameter must outlive the lifetime `'a` as defined on the associated item at 17:12
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());


error[E0478]: lifetime bound not satisfied
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
   |
   |
note: lifetime parameter instantiated with the lifetime `'a` as defined on the associated item at 17:12
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
   |            ^^
note: but lifetime parameter must outlive the lifetime `'b` as defined on the associated item at 17:16
   |
   |
LL |     type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     type C where Self: Copy = String;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Copy` for `Fooy<T>`
   |
LL | #[derive(Copy, Clone)]
   |          ^^^^
   |          ^^^^
note: the requirement `Fooy<T>: Copy` appears on the associated impl type `C` but not on the corresponding associated trait type
   |
LL | trait Foo {
   |       --- in this trait
...
...
LL |     type C where Self: Clone;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ this trait associated type doesn't have the requirement `Fooy<T>: Copy`
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> Foo for Fooy<T> {


error[E0277]: the trait bound `T: Copy` is not satisfied
   |
   |
LL |     fn d() where Self: Copy {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Copy` for `Fooy<T>`
   |
LL | #[derive(Copy, Clone)]
   |          ^^^^
   |          ^^^^
note: the requirement `Fooy<T>: Copy` appears on the impl method `d` but not on the corresponding trait method
   |
LL | trait Foo {
   |       --- in this trait
...
...
LL |     fn d() where Self: Clone;
   |        ^ this trait method doesn't have the requirement `Fooy<T>: Copy`
   = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
   |
LL | impl<T: std::marker::Copy> Foo for Fooy<T> {

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0310, E0478.
---
test result: FAILED. 12048 passed; 1 failed; 101 ignored; 0 measured; 0 filtered out; finished in 125.22s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:58
