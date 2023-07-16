plain
.................................................................................................... 9400/11762
.................................................................................................... 9500/11762
.............................................................................................i...... 9600/11762
i................................................................................................... 9700/11762
.......................................iiiiiii..iiiiii.i............................................ 9800/11762
.................................................................................................... 10000/11762
.................................................................................................... 10100/11762
.................................................................................................... 10200/11762
.................................................................................................... 10300/11762
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.10s

 finished in 0.160 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii...... 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.32s

 finished in 2.385 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

running 434 tests
i........................................i.........................F................................ 100/434
.................................................................................................... 200/434
..........F.................................................F...............F....F.................. 300/434
................................................i......................................F....F....... 400/434
failures:

---- [rustdoc] rustdoc/duplicate_impls/issue-33054.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/issue-33054" "/checkout/src/test/rustdoc/duplicate_impls/issue-33054.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
4: @count check failed
 Expected 1 occurrences but found 13
 // @count - '//*[@id="trait-implementations-list"]//*[@class="impl"]' 1
5: @count check failed
 Expected 1 occurrences but found 14
 // @count - '//*[@id="main"]//*[@class="impl"]' 1
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-21474.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21474" "/checkout/src/test/rustdoc/issue-21474.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @count check failed
 Expected 1 occurrences but found 13
 // @count issue_21474/struct.What.html '//*[@id="trait-implementations-list"]//*[@class="impl"]' 1
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-45584.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-45584" "/checkout/src/test/rustdoc/issue-45584.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @count check failed
 Expected 1 occurrences but found 13
 // @count - '//*[@id="trait-implementations-list"]//*[@class="impl"]' 1
13: @count check failed
 Expected 1 occurrences but found 13
 // @count - '//*[@id="trait-implementations-list"]//*[@class="impl"]' 1
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-50159.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-50159" "/checkout/src/test/rustdoc/issue-50159.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
17: @count check failed
 Expected 5 occurrences but found 12
 // @count - '//*[@id="synthetic-implementations-list"]//*[@class="impl"]' 5
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-53812.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53812" "/checkout/src/test/rustdoc/issue-53812.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="implementors-list"]//h3[2]' 'MyStruct<[T; 1]>'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="implementors-list"]//h3[3]' 'MyStruct<[T; 2]>'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="implementors-list"]//h3[4]' 'MyStruct<[T; 3]>'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="implementors-list"]//h3[5]' 'MyStruct<[T; 10]>'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/basic.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/basic" "/checkout/src/test/rustdoc/synthetic_auto/basic.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @count check failed
 Expected 5 occurrences but found 12
 // @count - '//*[@id="synthetic-implementations-list"]//*[@class="impl"]' 5
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/manual.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/manual" "/checkout/src/test/rustdoc/synthetic_auto/manual.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @count check failed
 Expected 1 occurrences but found 12
 // @count - '//*[@id="trait-implementations-list"]//*[@class="impl"]' 1
9: @count check failed
 Expected 4 occurrences but found 11
 // @count - '//*[@id="synthetic-implementations-list"]//*[@class="impl"]' 4
Encountered 2 errors

------------------------------------------

---
test result: FAILED. 424 passed; 7 failed; 3 ignored; 0 measured; 0 filtered out; finished in 36.81s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:54
