plain
test [ui] ui/derives/derives-span-Clone-enum-struct-variant.rs ... ok
test [ui] ui/derives/derives-span-Clone-tuple-struct.rs ... ok
test [ui] ui/deref-rc.rs ... ok
test [ui] ui/derives/derives-span-Debug-enum-struct-variant.rs ... ok
test [ui] ui/derives/derive-Debug-use-ufcs-struct.rs ... ok
test [ui] ui/derives/derives-span-Debug-struct.rs ... ok
test [ui] ui/derives/derives-span-Debug-enum.rs ... ok
test [ui] ui/derives/derives-span-Debug-enum.rs ... ok
test [ui] ui/derives/derive-Debug-use-ufcs-tuple.rs ... ok
test [ui] ui/derives/derive-hygiene.rs ... ok
test [ui] ui/derives/derives-span-Default-tuple-struct.rs ... ok
test [ui] ui/derives/derives-span-Debug-tuple-struct.rs ... ok
test [ui] ui/custom_test_frameworks/dynamic.rs ... ok
---
test [ui] ui/methods/method-call-lifetime-args-subst-index.rs ... ok
test [ui] ui/methods/method-argument-inference-associated-type.rs ... ok
test [ui] ui/meta/rustc-env.rs ... ok
test [ui] ui/methods/method-early-bound-lifetimes-on-self.rs ... ok
test [ui] ui/methods/method-lookup-order.rs#b00001 ... ok
test [ui] ui/methods/method-lookup-order.rs#b00010 ... ok
test [ui] ui/methods/method-lookup-order.rs#b00100 ... ok
test [ui] ui/methods/method-lookup-order.rs#b00011 ... ok
test [ui] ui/methods/method-lookup-order.rs#b00101 ... ok
test [ui] ui/methods/method-deref-to-same-trait-object-with-separate-params.rs ... ok
test [ui] ui/methods/method-lookup-order.rs#b00110 ... ok
test [ui] ui/methods/method-lookup-order.rs#b00111 ... ok
test [ui] ui/methods/method-lookup-order.rs#b01000 ... ok
test [ui] ui/methods/method-lookup-order.rs#b10000 ... ok
test [ui] ui/methods/method-lookup-order.rs#b01100 ... ok
test [ui] ui/methods/method-lookup-order.rs#b01101 ... ok
test [ui] ui/methods/method-lookup-order.rs#b01001 ... ok
test [ui] ui/methods/method-macro-backtrace.rs ... ok
test [ui] ui/methods/method-lookup-order.rs#b10010 ... ok
test [ui] ui/methods/method-lookup-order.rs#b10001 ... ok
test [ui] ui/methods/method-lookup-order.rs#b10011 ... ok
test [ui] ui/methods/method-lookup-order.rs#b10101 ... ok
test [ui] ui/methods/method-path-in-pattern.rs ... ok
test [ui] ui/methods/method-path-in-pattern.rs ... ok
test [ui] ui/methods/method-lookup-order.rs#b10111 ... ok
test [ui] ui/methods/method-lookup-order.rs#b11000 ... ok
test [ui] ui/methods/method-lookup-order.rs#b11001 ... ok
test [ui] ui/methods/method-normalize-bounds-issue-20604.rs ... ok
test [ui] ui/methods/method-lookup-order.rs#b11101 ... ok
test [ui] ui/methods/method-self-arg-2.rs ... ok
test [ui] ui/methods/method-on-ambiguous-numeric-type.rs ... ok
test [ui] ui/methods/method-recursive-blanket-impl.rs ... ok
test [ui] ui/methods/method-probe-no-guessing-dyn-trait.rs ... ok
---

---- [ui] ui/panic-handler/weak-lang-item.rs stdout ----
diff of stderr:

10 LL | extern crate core as other_core;
12 
12 
- error: `#[panic_handler]` function required, but not found
- 
15 error: language item required, but not found: `eh_personality`
+ 
+ error: `#[panic_handler]` function required, but not found
17 error: aborting due to 3 previous errors
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/panic-handler/weak-lang-item/weak-lang-item.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args panic-handler/weak-lang-item.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-handler/weak-lang-item.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/panic-handler/weak-lang-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/panic-handler/weak-lang-item/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0259]: the name `core` is defined multiple times
   |
LL | extern crate core;
LL | extern crate core;
   | ^^^^^^^^^^^^^^^^^^ `core` reimported here
   |
   = note: `core` must be defined only once in the type namespace of this module
help: you can use `as` to change the binding name of the import
   |
LL | extern crate core as other_core;


error: language item required, but not found: `eh_personality`

error: `#[panic_handler]` function required, but not found
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0259`.

---

Some tests failed in compiletest suite=ui mode=ui host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/bootstrap --exclude src/test/rustdoc-js --exclude src/tools/error_index_generator --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:26:12
