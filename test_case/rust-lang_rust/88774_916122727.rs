plain
test [ui] ui/expr-if-unique.rs ... ok
test [ui] ui/ext-nonexistent.rs ... ok
test [ui] ui/extenv/extenv-arg-2-not-string-literal.rs ... ok
test [ui] ui/extenv/extenv-not-defined-default.rs ... ok
test [ui] ui/expr/malformed_closure/ruby_style_closure.rs ... ok
test [ui] ui/extern-flag/empty-extern-arg.rs ... ok
test [ui] ui/extenv/extenv-not-defined-custom.rs ... ok
test [ui] ui/expr/if/expr-if-panic-pass.rs ... ok
test [ui] ui/extenv/extenv-too-many-args.rs ... ok
test [ui] ui/extenv/extenv-too-many-args.rs ... ok
test [ui] ui/expr/if/attrs/gate-whole-expr.rs ... ok
test [ui] ui/extenv/extenv-not-string-literal.rs ... ok
test [ui] ui/extern/extern-const.rs ... ignored
test [ui] ui/expr/malformed_closure/missing_braces_around_block.rs ... ok
test [ui] ui/expr/if/if-ret.rs ... ok
test [ui] ui/expr/if/if-check.rs ... ok
test [ui] ui/extern/extern-ffi-fn-with-body.rs ... ok
test [ui] ui/expr/compound-assignment/eval-order.rs ... ok
---
test [ui] ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] ui/crate-loading/invalid-so.rs stdout ----


- error[E0786]: found invalid metadata files for crate `bar`
+ error[E0463]: can't find crate for `bar`
2   --> $DIR/invalid-so.rs:3:7
3    |
4 LL | use ::bar;
-    |       ^^^
+    |       ^^^ can't find crate
6    |
6    |
-    = note: invalid metadata version found: $DIR/auxiliary/libbar.so
+    = note: extern location for bar is of an unknown type: $DIR/auxiliary/libbar.so
+    = help: file name should be lib*.rlib or *..wasm
9 error: aborting due to previous error
10 

- For more information about this error, try `rustc --explain E0786`.
- For more information about this error, try `rustc --explain E0786`.
+ For more information about this error, try `rustc --explain E0463`.
12 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/invalid-so/invalid-so.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/invalid-so.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/invalid-so.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/invalid-so" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--crate-type" "lib" "--extern" "bar=/checkout/src/test/ui/crate-loading/auxiliary/libbar.so" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/invalid-so/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `bar`
  --> /checkout/src/test/ui/crate-loading/invalid-so.rs:3:7
   |
LL | use ::bar; //~ ERROR invalid metadata files for crate `bar`
   |
   |
   = note: extern location for bar is of an unknown type: /checkout/src/test/ui/crate-loading/auxiliary/libbar.so
   = help: file name should be lib*.rlib or *..wasm
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:16:37
