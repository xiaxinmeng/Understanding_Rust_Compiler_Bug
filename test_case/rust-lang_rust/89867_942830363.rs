plain
warning: `tidy` is not installed; diffs will not be generated

running 476 tests
i...............................................i................................................... 100/476
.................................F................F..F......................F....................... 200/476
..................................................F................................................. 300/476
...............i...................F.F...........................................i.................. 400/476
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
........................................F..F...............................F

---- [rustdoc] rustdoc/inline_cross/macro-vis.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macro-vis" "/checkout/src/test/rustdoc/inline_cross/macro-vis.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
28: @has check failed
 File does not exist 'macro_vis/macro.this_is_dope.html'
 // @has macro_vis/macro.this_is_dope.html
29: @has check failed
 `XPATH PATTERN` did not match
 // @has macro_vis/index.html '//a/@href' 'macro.this_is_dope.html'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/basic.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/basic" "/checkout/src/test/rustdoc/intra-doc/basic.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' 'macro.this_macro.html'
49: @has check failed
 `XPATH PATTERN` did not match
 // @has basic/struct.ThisType.html '//a/@href' 'macro.this_macro.html'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/disambiguators-removed.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/disambiguators-removed" "/checkout/src/test/rustdoc/intra-doc/disambiguators-removed.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.Name.html"][code]' "Name"
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.Name.html"][code]' "Name!"
24: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.Name.html"]' "Name!"
32: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.Name.html"]' "Name"
42: @has check failed
 File does not exist 'disambiguators_removed/macro.Name.html'
 // @has disambiguators_removed/macro.Name.html
43: @has check failed
 File does not exist 'disambiguators_removed/macro.Name.html'
 // @has - '//a[@href="fn.Name.html"]' "fn@Name"
47: @has check failed
 File does not exist 'disambiguators_removed/macro.Name.html'
 // @has - '//a[@href="trait.Name.html"]' "trait@Name"
Encountered 7 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/macros-disambiguators.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/macros-disambiguators" "/checkout/src/test/rustdoc/intra-doc/macros-disambiguators.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="macro.foo.html"]' 'foo!()'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.foo.html"]' 'foo!{}'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.foo.html"]' 'foo![]'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.foo.html"]' 'foo1'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.foo.html"]' 'foo2'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="macro.foo.html"]' 'foo3'
Encountered 6 errors

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
5: @has check failed
 File does not exist 'issue_26606_macro/macro.make_item.html'
 // @has issue_26606_macro/macro.make_item.html
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/macro_rules-matchers.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/macro_rules-matchers" "/checkout/src/test/rustdoc/macro_rules-matchers.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
23: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has 'foo/macro.macro1.html'
24: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - 'macro_rules!'
25: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - 'macro1'
26: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - '{ () =&gt; { ... }; ($('
27: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - '//span[@class="macro-nonterminal"]' '$'
28: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - '//span[@class="macro-nonterminal"]' 'arg'
29: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - ':'
30: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - 'expr'
31: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - '),'
32: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - '+'
33: @has check failed
 File does not exist 'foo/macro.macro1.html'
     // @has - ') =&gt; { ... }; }'
Encountered 11 errors

------------------------------------------



---- [rustdoc] rustdoc/macros.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/macros" "/checkout/src/test/rustdoc/macros.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
1: @has check failed
 File does not exist 'macros/macro.my_macro.html'
 // @has macros/macro.my_macro.html //pre 'macro_rules! my_macro {'
2: @has check failed
 File does not exist 'macros/macro.my_macro.html'
 // @has - //pre '() => { ... };'
3: @has check failed
 File does not exist 'macros/macro.my_macro.html'
 // @has - //pre '($a : tt) => { ... };'
4: @has check failed
 File does not exist 'macros/macro.my_macro.html'
 // @has - //pre '($e : expr) => { ... };'
13: @has check failed
 File does not exist 'macros/macro.my_sub_macro.html'
 // @has macros/macro.my_sub_macro.html //pre 'macro_rules! my_sub_macro {'
14: @has check failed
 File does not exist 'macros/macro.my_sub_macro.html'
 // @has - //pre '() => { ... };'
15: @has check failed
 File does not exist 'macros/macro.my_sub_macro.html'
 // @has - //pre '($a : tt) => { ... };'
16: @has check failed
 File does not exist 'macros/macro.my_sub_macro.html'
 // @has - //pre '($e : expr) => { ... };'
Encountered 8 errors

------------------------------------------



---- [rustdoc] rustdoc/tab_title.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/tab_title" "/checkout/src/test/rustdoc/tab_title.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
29: @has check failed
 File does not exist 'foo/macro.cool_macro.html'
     // @has foo/macro.cool_macro.html '//head/title' 'cool_macro in foo - Rust'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/titles.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/titles" "/checkout/src/test/rustdoc/titles.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
30: @matches check failed
 File does not exist 'foo/macro.foo_macro.html'
 // @matches 'foo/macro.foo_macro.html' '//h1' 'Macro foo::foo_macro'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/without-redirect.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/without-redirect" "/checkout/src/test/rustdoc/without-redirect.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'foo/macro.bar.html'
 // @has foo/macro.bar.html
4: @has check failed
 File does not exist 'foo/macro.bar!.html'
 // @has foo/macro.bar!.html
Encountered 2 errors

------------------------------------------

---
test result: FAILED. 462 passed; 10 failed; 4 ignored; 0 measured; 0 filtered out; finished in 42.64s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:16:48
