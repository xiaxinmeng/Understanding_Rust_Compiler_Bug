plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 443 tests
i..............F.........................i.......................................................... 100/443
..........F.F....................................................................................... 200/443
................F..................................i................................................ 400/443
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........................................
failures:
failures:

---- [rustdoc] rustdoc/assoc-consts.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
91: @!has check failed
 `XPATH PATTERN` did not match
     // @!has - '//div[@class="impl-items"]/details[@open=""]//*[@class="docblock"]' "Docs for QUX_DEFAULT12 in trait."
99: @!has check failed
 `XPATH PATTERN` did not match
     // @!has - '//div[@class="impl-items"]/details[@open=""]//*[@class="docblock"]' "Docs for QUX_DEFAULT2 in trait."
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//details/details/div[@class="docblock"]' 'docs for ConstWithDefault'
25: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//details/details/div[@class="docblock"]' 'docs for TypeWithDefault'
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//details/details/div[@class="docblock"]' 'docs for method_with_default'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/impl-inline-without-trait.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/impl-inline-without-trait" "/checkout/src/test/rustdoc/inline_cross/impl-inline-without-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//details/details/div[@class="docblock"]' 'docs for my_trait_method'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/manual_impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/manual_impl" "/checkout/src/test/rustdoc/manual_impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
27: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//div[@class="impl-items"]/details[@open=""]//div[@class="docblock"]' 'Docs associated with the trait b_method definition.'
29: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//div[@class="impl-items"]/details[@open=""]//div[@class="docblock"]' 'Docs associated with the trait c_method definition.'
32: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//div[@class="impl-items"]/details[@open=""]//div[@class="docblock"]' 'Read more'
50: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//div[@class="impl-items"]/details[@open=""]//div[@class="docblock"]' 'Docs associated with the trait b_method definition.'
71: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//div[@class="impl-items"]/details[@open=""]//div[@class="docblock"]' 'Docs associated with the trait a_method definition.'
Encountered 5 errors

------------------------------------------

---
test result: FAILED. 436 passed; 4 failed; 3 ignored; 0 measured; 0 filtered out; finished in 31.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:50
