plain
diff of stderr:

15    |
16 
17 error[E0599]: the method `extend_from_slice` exists for mutable reference `&mut Vec<SomeDerives>`, but its trait bounds were not satisfied
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/issue-91492.rs:11:9
19    |
19    |
20 LL | pub struct SomeDerives;
21    | ----------------------- doesn't satisfy `SomeDerives: Clone`
31    |
32 
32 
33 error[E0599]: the method `use_clone` exists for struct `Object<NoDerives, SomeDerives>`, but its trait bounds were not satisfied
-   --> $DIR/issue-91492.rs:20:9
35    |
35    |
36 LL | pub struct NoDerives;
37    | --------------------- doesn't satisfy `NoDerives: Clone`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492/issue-91492.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/issue-91492.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/issue-91492.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `extend_from_slice` exists for mutable reference `&mut Vec<NoDerives>`, but its trait bounds were not satisfied
  --> /checkout/src/test/ui/derives/issue-91492.rs:4:9
   |
LL | pub struct NoDerives;
   | --------------------- doesn't satisfy `NoDerives: Clone`
LL | fn fun1(foo: &mut Vec<NoDerives>, bar: &[NoDerives]) {
LL |     foo.extend_from_slice(bar); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
           `NoDerives: Clone`
           `NoDerives: Clone`
help: consider annotating `NoDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0599]: the method `extend_from_slice` exists for mutable reference `&mut Vec<SomeDerives>`, but its trait bounds were not satisfied
  --> /checkout/src/test/ui/derives/issue-91492.rs:12:9
   |
LL | pub struct SomeDerives;
   | ----------------------- doesn't satisfy `SomeDerives: Clone`
LL | fn fun2(foo: &mut Vec<SomeDerives>, bar: &[SomeDerives]) {
LL |     foo.extend_from_slice(bar); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `SomeDerives: Clone`
help: consider annotating `SomeDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0599]: the method `use_clone` exists for struct `Object<NoDerives, SomeDerives>`, but its trait bounds were not satisfied
  --> /checkout/src/test/ui/derives/issue-91492.rs:22:9
   |
LL | pub struct NoDerives;
   | --------------------- doesn't satisfy `NoDerives: Clone`
...
LL | struct Object<T, A>(T, A);
   | -------------------------- method `use_clone` not found for this
...
LL |     foo.use_clone(); //~ ERROR
   |         ^^^^^^^^^ method cannot be called on `Object<NoDerives, SomeDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `NoDerives: Clone`
           `NoDerives: Clone`
help: consider annotating `NoDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 3 previous errors
---
test result: FAILED. 12338 passed; 1 failed; 118 ignored; 0 measured; 0 filtered out; finished in 141.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:33
