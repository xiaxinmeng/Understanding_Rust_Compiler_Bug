plain

---- [ui] ui/feature-gates/feature-gate-cfg-target-abi.rs stdout ----
diff of stderr:

- error[E0658]: `cfg(target_abi)` is experimental and subject to change (see issue #80970)
+ error[E0658]: `cfg(target_abi)` is experimental and subject to change
3    |
3    |
- LL | #[cfg_attr(target_abi = "x", x)] //~ ERROR `cfg(target_abi)` is experimental
+ LL | #[cfg_attr(target_abi = "x", x)]
6    |
6    |
-    = help: add #![feature(cfg_target_abi)] to the crate attributes to enable
+    = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
+    = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable
8 
- error[E0658]: `cfg(target_abi)` is experimental and subject to change (see issue #80970)
+ error[E0658]: `cfg(target_abi)` is experimental and subject to change
11    |
11    |
- LL | #[cfg(target_abi = "x")] //~ ERROR `cfg(target_abi)` is experimental
+ LL | #[cfg(target_abi = "x")]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
14    |
14    |
-    = help: add #![feature(cfg_target_abi)] to the crate attributes to enable
+    = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
+    = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable
16 
- error[E0658]: `cfg(target_abi)` is experimental and subject to change (see issue #80970)
+ error[E0658]: `cfg(target_abi)` is experimental and subject to change
19    |
19    |
- LL | #[cfg(not(any(all(target_abi = "x"))))] //~ ERROR `cfg(target_abi)` is experimental
+ LL | #[cfg(not(any(all(target_abi = "x"))))]
22    |
22    |
-    = help: add #![feature(cfg_target_abi)] to the crate attributes to enable
+    = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
+    = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable
24 
- error[E0658]: `cfg(target_abi)` is experimental and subject to change (see issue #80970)
+ error[E0658]: `cfg(target_abi)` is experimental and subject to change
27    |
27    |
28 LL |     cfg!(target_abi = "x");
29    |          ^^^^^^^^^^^^^^^^
30    |
30    |
-    = help: add #![feature(cfg_target_abi)] to the crate attributes to enable
+    = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
+    = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable
33 error: aborting due to 4 previous errors
34 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-abi/feature-gate-cfg-target-abi.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-cfg-target-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg-target-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-target-abi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL | #[cfg_attr(target_abi = "x", x)] //~ ERROR `cfg(target_abi)` is experimental
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable

error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL | #[cfg(target_abi = "x")] //~ ERROR `cfg(target_abi)` is experimental
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable

error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL | #[cfg(not(any(all(target_abi = "x"))))] //~ ERROR `cfg(target_abi)` is experimental
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable

error[E0658]: `cfg(target_abi)` is experimental and subject to change
   |
   |
LL |     cfg!(target_abi = "x");
   |
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = note: see issue #80970 <https://github.com/rust-lang/rust/issues/80970> for more information
   = help: add `#![feature(cfg_target_abi)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 11960 passed; 1 failed; 100 ignored; 0 measured; 0 filtered out; finished in 126.38s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:16
