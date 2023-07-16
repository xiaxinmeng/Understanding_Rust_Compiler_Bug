plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.064 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.52s

 finished in 2.591 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; generated diffs will be harder to read

running 427 tests
.............F........................i..................................................F.......... 100/427
.................................................................................................F.. 300/427
.................................................................................................F.. 300/427
..................................F.........i............................F.......................... 400/427
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----
---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.S.html '//h3[@id="impl-Into%3CU%3E"]//code' 'impl<T, U> Into<U> for T'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/blanket-reexport-item.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item.nightly" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"`', src/tools/compiletest/src/runtest.rs:1872:33

---- [rustdoc] rustdoc/generic-impl.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//h3[@id="impl-ToString"]//code' 'impl<T> ToString for T'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//div[@class="sidebar-links"]/a[@href="#impl-ToString"]' 'ToString'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/generic-impl.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl.nightly" "/checkout/src/test/rustdoc/generic-impl.rs"`', src/tools/compiletest/src/runtest.rs:1872:33
---- [rustdoc] rustdoc/issue-78673.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-78673" "/checkout/src/test/rustdoc/issue-78673.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl"]' 'AnAmazingTrait for T'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/issue-78673.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-78673/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-78673.nightly" "/checkout/src/test/rustdoc/issue-78673.rs"`', src/tools/compiletest/src/runtest.rs:1872:33
---- [rustdoc] rustdoc/primitive-generic-impl.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive-generic-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/primitive.i32.html '//h3[@id="impl-ToString"]//code' 'impl<T> ToString for T'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/primitive-generic-impl.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl.nightly" "/checkout/src/test/rustdoc/primitive-generic-impl.rs"`', src/tools/compiletest/src/runtest.rs:1872:33
---- [rustdoc] rustdoc/src-links-auto-impls.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-auto-impls" "/checkout/src/test/rustdoc/src-links-auto-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@id="impl-Any"]/code' 'impl<T> Any for T'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@id="impl-Any"]/a[@class="srclink"]' '[src]'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/src-links-auto-impls.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-auto-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-auto-impls.nightly" "/checkout/src/test/rustdoc/src-links-auto-impls.rs"`', src/tools/compiletest/src/runtest.rs:1872:33

failures:
    [rustdoc] rustdoc/blanket-reexport-item.rs
    [rustdoc] rustdoc/generic-impl.rs
---
test result: FAILED. 420 passed; 5 failed; 2 ignored; 0 measured; 0 filtered out; finished in 34.84s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:42
