plain

---- [ui] ui/auto-traits/suspicious-impls-lint.rs stdout ----
diff of stderr:

10 LL | #![deny(suspicious_auto_trait_impls)]
12    = warning: this will change its meaning in a future release!
-    = note: for more information, see issue #89460 <https://github.com/rust-lang/rust/issues/89460>
+    = note: for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>
14 note: try using the same sequence of generic parameters as the struct definition
---
To only update this specific test, also pass `--test-args auto-traits/suspicious-impls-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/auto-traits/suspicious-impls-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/auto-traits/suspicious-impls-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/auto-traits/suspicious-impls-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cross-crate traits with a default impl, like `Send`, should not be specialized
   |
   |
LL | unsafe impl<T: Send> Send for MayImplementSendErr<&T> {}
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/auto-traits/suspicious-impls-lint.rs:1:9
   |
   |
LL | #![deny(suspicious_auto_trait_impls)]
   = warning: this will change its meaning in a future release!
   = note: for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>
note: try using the same sequence of generic parameters as the struct definition
  --> /checkout/src/test/ui/auto-traits/suspicious-impls-lint.rs:6:1
  --> /checkout/src/test/ui/auto-traits/suspicious-impls-lint.rs:6:1
   |
LL | struct MayImplementSendErr<T>(T);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `&T` is not a generic parameter

error: cross-crate traits with a default impl, like `Send`, should not be specialized
   |
   |
LL | unsafe impl Send for ContainsVec<i32> {}
   |
   = warning: this will change its meaning in a future release!
   = note: for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>
note: try using the same sequence of generic parameters as the struct definition
note: try using the same sequence of generic parameters as the struct definition
  --> /checkout/src/test/ui/auto-traits/suspicious-impls-lint.rs:18:1
   |
LL | struct ContainsVec<T>(Vec<T>);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `i32` is not a generic parameter

error: cross-crate traits with a default impl, like `Send`, should not be specialized
   |
   |
LL | unsafe impl<T: Send> Send for TwoParamsSame<T, T> {}
   |
   = warning: this will change its meaning in a future release!
   = note: for more information, see issue #93367 <https://github.com/rust-lang/rust/issues/93367>
note: try using the same sequence of generic parameters as the struct definition
note: try using the same sequence of generic parameters as the struct definition
  --> /checkout/src/test/ui/auto-traits/suspicious-impls-lint.rs:29:1
   |
LL | struct TwoParamsSame<T, U>(T, U);
   = note: `T` is mentioned multiple times

error: aborting due to 3 previous errors

---
test result: FAILED. 12434 passed; 1 failed; 121 ignored; 0 measured; 0 filtered out; finished in 100.65s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:34
