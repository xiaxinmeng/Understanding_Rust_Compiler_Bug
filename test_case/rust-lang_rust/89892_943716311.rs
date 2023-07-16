plain
1 error[E0308]: mismatched types
-   --> $DIR/return-impl-trait.rs:10:5
+   --> $DIR/return-impl-trait.rs:13:5
3    |
- LL | fn bar<T: Trait + std::marker::Sync>() -> T where T: Send {
+ LL | fn bar<T: Trait + std::marker::Sync>() -> T
5    |        -                                  -
7    |        |                                  expected `T` because of return type

8    |        this type parameter                help: consider using an impl return type: `impl Trait + std::marker::Sync + Send`
+ ...
---
15 error[E0308]: mismatched types
-   --> $DIR/return-impl-trait.rs:14:6
+   --> $DIR/return-impl-trait.rs:17:5
17    |
18 LL | fn bad_echo<T>(_t: T) -> T {
19    |             -            - expected `T` because of return type
20    |             |
21    |             this type parameter
- LL |      "this should not suggest impl Trait"
-    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
---
28 error[E0308]: mismatched types
-   --> $DIR/return-impl-trait.rs:18:6
+   --> $DIR/return-impl-trait.rs:21:5
30    |
31 LL | fn bad_echo_2<T: Trait>(_t: T) -> T {
32    |               -                   - expected `T` because of return type
33    |               |
34    |               this type parameter
- LL |      "this should not suggest impl Trait"
-    |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
---
To only update this specific test, also pass `--test-args return/return-impl-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/return/return-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/return/return-impl-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-impl-trait.rs:13:5
   |
LL | fn bar<T: Trait + std::marker::Sync>() -> T
   |        -                                  -
   |        |                                  expected `T` because of return type
   |        this type parameter                help: consider using an impl return type: `impl Trait + std::marker::Sync + Send`
...
...
LL |     () //~ ERROR mismatched types
   |     ^^ expected type parameter `T`, found `()`
   = note: expected type parameter `T`
                   found unit type `()`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-impl-trait.rs:17:5
   |
LL | fn bad_echo<T>(_t: T) -> T {
   |             -            - expected `T` because of return type
   |             this type parameter
   |             this type parameter
LL |     "this should not suggest impl Trait" //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
   = note: expected type parameter `T`
                   found reference `&'static str`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/return/return-impl-trait.rs:21:5
   |
LL | fn bad_echo_2<T: Trait>(_t: T) -> T {
   |               -                   - expected `T` because of return type
   |               this type parameter
   |               this type parameter
LL |     "this should not suggest impl Trait" //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type parameter `T`, found `&str`
   = note: expected type parameter `T`
                   found reference `&'static str`

error: aborting due to 3 previous errors
---
test result: FAILED. 12175 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 132.54s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:29
