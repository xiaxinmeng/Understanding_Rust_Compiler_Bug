plain
+    |
+ LL | impl const std::ops::Add for i32 {
+    |            ^^^^^^^^^^^^^ cannot infer type for type `i32`
+    |
+    = note: cannot satisfy `i32: Add`
+ note: required by a bound in `Add`
+   --> $SRC_DIR/core/src/ops/arith.rs:LL:COL
+    |
+ LL | / pub trait Add<Rhs = Self> {
+ LL | |     /// The resulting type after applying the `+` operator.
+ LL | |     #[stable(feature = "rust1", since = "1.0.0")]
+ LL | |     type Output;
+ ...  |
+ LL | |     fn add(self, rhs: Rhs) -> Self::Output;
+ LL | | }
+    | |_^ required by this bound in `Add`
- Some errors have detailed explanations: E0117, E0119.
+ error[E0283]: type annotations needed
+   --> $DIR/const-and-non-const-impl.rs:14:6
+    |
+    |
+ LL | impl std::ops::Add for Int {
+    |      ^^^^^^^^^^^^^ cannot infer type for struct `Int`
+    |
+    = note: cannot satisfy `Int: Add`
+ note: required by a bound in `Add`
+   --> $SRC_DIR/core/src/ops/arith.rs:LL:COL
+    |
+ LL | / pub trait Add<Rhs = Self> {
+ LL | |     /// The resulting type after applying the `+` operator.
+ LL | |     #[stable(feature = "rust1", since = "1.0.0")]
+ LL | |     type Output;
+ ...  |
+ LL | |     fn add(self, rhs: Rhs) -> Self::Output;
+ LL | | }
+    | |_^ required by this bound in `Add`
+ error[E0283]: type annotations needed
+   --> $DIR/const-and-non-const-impl.rs:22:12
+    |
+ LL | impl const std::ops::Add for Int {
+ LL | impl const std::ops::Add for Int {
+    |            ^^^^^^^^^^^^^ cannot infer type for struct `Int`
+    |
+    = note: cannot satisfy `Int: Add`
+ note: required by a bound in `Add`
+   --> $SRC_DIR/core/src/ops/arith.rs:LL:COL
+    |
+ LL | / pub trait Add<Rhs = Self> {
+ LL | |     /// The resulting type after applying the `+` operator.
+ LL | |     #[stable(feature = "rust1", since = "1.0.0")]
+ LL | |     type Output;
+ ...  |
+ LL | |     fn add(self, rhs: Rhs) -> Self::Output;
+ LL | | }
+    | |_^ required by this bound in `Add`
+ error: aborting due to 5 previous errors
+ 
+ Some errors have detailed explanations: E0117, E0119, E0283.
25 For more information about an error, try `rustc --explain E0117`.
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-and-non-const-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-and-non-const-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-and-non-const-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-and-non-const-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
   |
LL | impl const std::ops::Add for i32 {
   | ^^^^^^^^^^^-------------^^^^^---
   | |          |                 |
   | |          |                 |
   | |          |                 `i32` is not defined in the current crate
   | |          `i32` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   = note: define and implement a trait or new type instead

error[E0119]: conflicting implementations of trait `std::ops::Add` for type `Int`
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-and-non-const-impl.rs:22:1
---
   |
LL | impl const std::ops::Add for i32 {
   |            ^^^^^^^^^^^^^ cannot infer type for type `i32`
   |
   = note: cannot satisfy `i32: Add`
note: required by a bound in `Add`
   |
   |
LL | / pub trait Add<Rhs = Self> {
LL | |     /// The resulting type after applying the `+` operator.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Output;
...  |
LL | |     fn add(self, rhs: Rhs) -> Self::Output;
LL | | }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   | |_^ required by this bound in `Add`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-and-non-const-impl.rs:14:6
   |
LL | impl std::ops::Add for Int {
LL | impl std::ops::Add for Int {
   |      ^^^^^^^^^^^^^ cannot infer type for struct `Int`
   |
   = note: cannot satisfy `Int: Add`
note: required by a bound in `Add`
  --> /checkout/library/core/src/ops/arith.rs:75:1
   |
LL | / pub trait Add<Rhs = Self> {
LL | |     /// The resulting type after applying the `+` operator.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Output;
...  |
LL | |     fn add(self, rhs: Rhs) -> Self::Output;
LL | | }
   | |_^ required by this bound in `Add`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-and-non-const-impl.rs:22:12
   |
LL | impl const std::ops::Add for Int {
LL | impl const std::ops::Add for Int {
   |            ^^^^^^^^^^^^^ cannot infer type for struct `Int`
   |
   = note: cannot satisfy `Int: Add`
note: required by a bound in `Add`
  --> /checkout/library/core/src/ops/arith.rs:75:1
   |
LL | / pub trait Add<Rhs = Self> {
LL | |     /// The resulting type after applying the `+` operator.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Output;
...  |
LL | |     fn add(self, rhs: Rhs) -> Self::Output;
LL | | }
   | |_^ required by this bound in `Add`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0117, E0119, E0283.
For more information about an error, try `rustc --explain E0117`.
---
test result: FAILED. 12192 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 132.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:33
