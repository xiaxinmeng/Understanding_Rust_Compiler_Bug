plain
.....................................................i.i............................................ 12000/12062
..............................................................
failures:

---- [ui] ui/consts/rustc-const-stability-require-const.rs stdout ----


96 LL | pub extern "stdcall" fn foo_stdcall() {}
98 
- error: aborting due to 7 previous errors
+ warning: use of calling convention not supported on this target
+   --> $DIR/rustc-const-stability-require-const.rs:43:1
+   --> $DIR/rustc-const-stability-require-const.rs:43:1
+    |
+ LL | pub extern "stdcall" fn foo_stdcall() {}
+    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    = note: `#[warn(unsupported_calling_conventions)]` on by default
+    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
---
To only update this specific test, also pass `--test-args consts/rustc-const-stability-require-const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/rustc-const-stability-require-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rustc-const-stability-require-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/rustc-const-stability-require-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:7:1
   |
LL | #[rustc_const_unstable(feature = "const_foo", issue = "none")]
   | -------------------------------------------------------------- attribute specified here
LL | pub fn foo() {}
   |
help: make the function or method const
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:7:1
   |
   |
LL | pub fn foo() {}
   | ^^^^^^^^^^^^

error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:12:1
   |
LL | #[rustc_const_stable(feature = "const_bar", since = "1.0.0")]
   | ------------------------------------------------------------- attribute specified here
LL | pub fn bar() {}
   |
help: make the function or method const
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:12:1
   |
   |
LL | pub fn bar() {}
   | ^^^^^^^^^^^^

error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:21:5
   |
LL |     #[rustc_const_unstable(feature = "const_salad", issue = "none")]
   |     ---------------------------------------------------------------- attribute specified here
LL |     pub fn salad(&self) -> &'static str { "mmmmmm" }
   |
help: make the function or method const
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:21:5
   |
   |
LL |     pub fn salad(&self) -> &'static str { "mmmmmm" }


error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:26:5
   |
LL |     #[rustc_const_unstable(feature = "const_roasted", issue = "none")]
   |     ------------------------------------------------------------------ attribute specified here
LL |     pub fn roasted(&self) -> &'static str { "mmmmmmmmmm" }
   |
help: make the function or method const
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:26:5
   |
   |
LL |     pub fn roasted(&self) -> &'static str { "mmmmmmmmmm" }


error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:32:1
   |
LL | #[rustc_const_stable(feature = "const_bar", since = "1.0.0")]
   | ------------------------------------------------------------- attribute specified here
LL | pub extern "C" fn bar_c() {}
   |
help: make the function or method const
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:32:1
   |
   |
LL | pub extern "C" fn bar_c() {}


error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:37:1
   |
LL | #[rustc_const_unstable(feature = "const_foo", issue = "none")]
   | -------------------------------------------------------------- attribute specified here
LL | pub extern "C" fn foo_c() {}
   |
help: make the function or method const
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:37:1
   |
   |
LL | pub extern "C" fn foo_c() {}


error: attributes `#[rustc_const_unstable]` and `#[rustc_const_stable]` require the function or method to be `const`
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:43:1
   |
LL | #[rustc_const_unstable(feature = "const_foo", issue = "none")]
   | -------------------------------------------------------------- attribute specified here
LL | pub extern "stdcall" fn foo_stdcall() {}
   |
help: make the function or method const
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:43:1
   |
   |
LL | pub extern "stdcall" fn foo_stdcall() {}

warning: use of calling convention not supported on this target
  --> /checkout/src/test/ui/consts/rustc-const-stability-require-const.rs:43:1
   |
   |
LL | pub extern "stdcall" fn foo_stdcall() {}
   |
   = note: `#[warn(unsupported_calling_conventions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #00000 <https://github.com/rust-lang/rust/issues/00000>
---
test result: FAILED. 11961 passed; 1 failed; 100 ignored; 0 measured; 0 filtered out; finished in 125.76s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:20
