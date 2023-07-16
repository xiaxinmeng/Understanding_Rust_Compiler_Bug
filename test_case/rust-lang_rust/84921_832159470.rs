plain
.................................................................................................... 9400/11823
.................................................................................................... 9500/11823
.................................................................................................... 9600/11823
...........................................i......i................................................. 9700/11823
.........................................................................................iiiiiii..ii 9800/11823
.................................................................................................... 10000/11823
.................................................................................................... 10100/11823
.................................................................................................... 10200/11823
.................................................................................................... 10300/11823
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.158 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.360 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 439 tests
i.........................F..............i.......................................................... 100/439
......................FF..............................................................F............. 200/439
.....................................................................................F............F. 300/439
.........................FF.......................i..........................................F...... 400/439
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....................F..................

---- [rustdoc] rustdoc/const-generics/add-impl.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/add-impl" "/checkout/src/test/rustdoc/const-generics/add-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Simd.html '//div[@id="trait-implementations-list"]//h3/code' 'impl Add<Simd<u8, 16_usize>> for Simd<u8, 16>'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/issue-32881.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-32881" "/checkout/src/test/rustdoc/inline_cross/issue-32881.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' "impl<'a> Debug for dyn Bar"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/trait-vis.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/trait-vis" "/checkout/src/test/rustdoc/inline_cross/trait-vis.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'impl Clone for SomeStruct'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-15169.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-15169" "/checkout/src/test/rustdoc/issue-15169.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
1: @has check failed
 `XPATH PATTERN` did not match
 // @has issue_15169/struct.Foo.html '//*[@id="method.eq"]' 'fn eq'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-55321.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55321" "/checkout/src/test/rustdoc/issue-55321.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="trait-implementations-list"]//*[@class="impl"]//code' "impl !Send for A"
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="trait-implementations-list"]//*[@class="impl"]//code' "impl !Sync for A"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-72340.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-72340" "/checkout/src/test/rustdoc/issue-72340.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 `XPATH PATTERN` did not match
     // @has foo/struct.Body.html '//a/@href' 'struct.Body.html#method.empty'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/negative-impl-sidebar.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/negative-impl-sidebar" "/checkout/src/test/rustdoc/negative-impl-sidebar.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"][@href="#trait-implementations"]' 'Trait Implementations'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/negative-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/negative-impl" "/checkout/src/test/rustdoc/negative-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @matches check failed
 `XPATH PATTERN` did not match
 // @matches negative_impl/struct.Alpha.html '//*[@class="impl"]//code' "impl !Send for Alpha"
11: @matches check failed
 `XPATH PATTERN` did not match
 // @matches negative_impl/struct.Bravo.html '//*[@class="impl"]//code' "impl<B> !Send for Bravo<B>"
Encountered 2 errors

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
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="trait-implementations-list"]//*[@class="impl"]//code' 'impl<T> Send for Foo<T>'
8: @count check failed
 Expected 1 occurrences but found 0
 // @count - '//*[@id="trait-implementations-list"]//*[@class="impl"]' 1
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/traits-in-bodies.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/traits-in-bodies" "/checkout/src/test/rustdoc/traits-in-bodies.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'impl Clone for SomeStruct'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'impl Copy for Point'
34: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'impl Clone for Inception'
Encountered 3 errors

------------------------------------------

---
test result: FAILED. 426 passed; 10 failed; 3 ignored; 0 measured; 0 filtered out; finished in 36.43s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:20
