plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 490 tests
i....................................................i...........................................F.. 100/490
...F................................................................................................ 200/490
.....................FF....................................F........................................ 300/490
............................i.................................................................i..... 400/490
........................F..............................F.......F..........................
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/ensure-src-link.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ensure-src-link" "/checkout/src/test/rustdoc/ensure-src-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/trait.Iterator.html '//div[@id="method.zip"]//a[@class="srclink"]' "[src]"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/external-macro-src.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/external-macro-src" "/checkout/src/test/rustdoc/external-macro-src.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="../src/foo/external-macro-src.rs.html#3-12"]' '[src]'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../src/foo/external-macro-src.rs.html#12"]' '[src]'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-16265-1.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-1" "/checkout/src/test/rustdoc/issue-16265-1.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `PATTERN` did not match
 // @has issue_16265_1/traits/index.html '[src]'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-16265-2.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-16265-2" "/checkout/src/test/rustdoc/issue-16265-2.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
1: @has check failed
 `PATTERN` did not match
 // @has issue_16265_2/index.html '[src]'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-26606.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26606" "/checkout/src/test/rustdoc/issue-26606.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../src/issue_26606/issue-26606.rs.html#11"]' '[src]'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/src-links-auto-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/src-links-auto-impls" "/checkout/src/test/rustdoc/src-links-auto-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Any"]//a[@class="srclink"]' '[src]'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/thread-local-src.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/thread-local-src" "/checkout/src/test/rustdoc/thread-local-src.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="../src/foo/thread-local-src.rs.html#1-6"]' '[src]'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/constant.FOO.html '//a[@href="../src/foo/thread-local-src.rs.html#6"]' '[src]'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/trait-src-link.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-src-link" "/checkout/src/test/rustdoc/trait-src-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
     // @has quix/trait.Foo.html '//a[@href="../src/quix/trait-src-link.rs.html#4"]' '[src]'
6: @has check failed
 `XPATH PATTERN` did not match
     // @has quix/trait.Foo.html '//a[@href="../src/quix/trait-src-link.rs.html#7"]' '[src]'
13: @has check failed
 `XPATH PATTERN` did not match
     // @has quix/struct.Bar.html '//a[@href="../src/quix/trait-src-link.rs.html#14"]' '[src]'
15: @has check failed
 `XPATH PATTERN` did not match
     // @has quix/struct.Bar.html '//a[@href="../src/quix/trait-src-link.rs.html#7"]' '[src]'
21: @has check failed
 `XPATH PATTERN` did not match
     // @has quix/struct.Baz.html '//a[@href="../src/quix/trait-src-link.rs.html#22"]' '[src]'
24: @has check failed
 `XPATH PATTERN` did not match
     // @has quix/struct.Baz.html '//a[@href="../src/quix/trait-src-link.rs.html#25"]' '[src]'
Encountered 6 errors

------------------------------------------

---
test result: FAILED. 478 passed; 8 failed; 4 ignored; 0 measured; 0 filtered out; finished in 46.11s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:17:12
