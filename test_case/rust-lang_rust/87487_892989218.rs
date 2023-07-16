plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 458 tests
i...........F..............................i................................................F....... 100/458
...............................FF.F.F............................................................... 200/458
...................................................F................................................ 300/458
....................................F.........................F.i..F...F........F..F................ 400/458
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..F.......................................................

---- [rustdoc] rustdoc/all.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/all" "/checkout/src/test/rustdoc/all.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
26: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/all.html '//a[@href="struct.ReexportedStruct.html"]' 'ReexportedStruct'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/foreigntype-reexport.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/foreigntype-reexport" "/checkout/src/test/rustdoc/foreigntype-reexport.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
32: @has check failed
 File does not exist 'foreigntype_reexport/foreigntype.C2.html'
 // @has foreigntype_reexport/foreigntype.C2.html
33: @has check failed
 File does not exist 'foreigntype_reexport/fn.f2.html'
 // @has foreigntype_reexport/fn.f2.html
34: @has check failed
 File does not exist 'foreigntype_reexport/static.K2.html'
 // @has foreigntype_reexport/static.K2.html
35: @has check failed
 `XPATH PATTERN` did not match
 // @has foreigntype_reexport/index.html '//a[@class="foreigntype"]' 'C2'
36: @has check failed
 `XPATH PATTERN` did not match
 // @has foreigntype_reexport/index.html '//a[@class="fn"]' 'f2'
37: @has check failed
 `XPATH PATTERN` did not match
 // @has foreigntype_reexport/index.html '//a[@class="static"]' 'K2'
Encountered 6 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_local/issue-32343.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-32343" "/checkout/src/test/rustdoc/inline_local/issue-32343.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code/a' 'Bar'
21: @has check failed
 File does not exist 'issue_32343/bar/struct.Bar.html'
     // @has issue_32343/bar/struct.Bar.html
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_local/issue-28537.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/issue-28537" "/checkout/src/test/rustdoc/inline_local/issue-28537.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
16: @has check failed
 File does not exist 'issue_28537/struct.Bar.html'
 // @has issue_28537/struct.Bar.html
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/renamed-via-module.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/renamed-via-module" "/checkout/src/test/rustdoc/inline_cross/renamed-via-module.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/[@href="struct.DeprecatedStepBy.html"]' "DeprecatedStepBy"
12: @has check failed
 File does not exist 'foo/iter/struct.DeprecatedStepBy.html'
 // @has foo/iter/struct.DeprecatedStepBy.html
13: @has check failed
 File does not exist 'foo/iter/struct.DeprecatedStepBy.html'
 // @has - '//h1' "Struct foo::iter::DeprecatedStepBy"
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_local/trait-vis.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/trait-vis" "/checkout/src/test/rustdoc/inline_local/trait-vis.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @has check failed
 File does not exist 'trait_vis/struct.SomeStruct.html'
 // @has trait_vis/struct.SomeStruct.html
16: @has check failed
 File does not exist 'trait_vis/struct.SomeStruct.html'
 // @has - '//h3[@class="code-header in-band"]' 'impl ThisTrait for SomeStruct'
17: @!has check failed
 File does not exist 'trait_vis/struct.SomeStruct.html'
 // @!has - '//h3[@class="code-header in-band"]' 'impl PrivateTrait for SomeStruct'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-34473.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34473" "/checkout/src/test/rustdoc/issue-34473.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @!has check failed
 `PATTERN` did not match
 // @!has - SomeTypeWithLongName
9: @has check failed
 File does not exist 'foo/struct.SomeType.html'
 // @has foo/struct.SomeType.html
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/macro_pub_in_module.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/macro_pub_in_module" "/checkout/src/test/rustdoc/macro_pub_in_module.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 File does not exist 'external_crate/some_module/macro.external_macro.html'
  // @has external_crate/some_module/macro.external_macro.html
35: @has check failed
 File does not exist 'krate/inner/macro.unrenamed.html'
     // @has krate/inner/macro.unrenamed.html
42: @has check failed
 File does not exist 'krate/inner/macro.renamed.html'
     // @has krate/inner/macro.renamed.html
51: @has check failed
 File does not exist 'krate/inner/macro.m2.html'
     // @has krate/inner/macro.m2.html
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/redirect-const.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect-const" "/checkout/src/test/rustdoc/redirect-const.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 File does not exist 'foo/hidden/static.STATIC_FOO.html'
     // @has foo/hidden/static.STATIC_FOO.html
8: @has check failed
 File does not exist 'foo/hidden/static.STATIC_FOO.html'
     // @has - '//p/a' '../../foo/static.STATIC_FOO.html'
10: @has check failed
 File does not exist 'foo/hidden/constant.CONST_FOO.html'
     // @has foo/hidden/constant.CONST_FOO.html
11: @has check failed
 File does not exist 'foo/hidden/constant.CONST_FOO.html'
     // @has - '//p/a' '../../foo/constant.CONST_FOO.html'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/redirect-rename.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect-rename" "/checkout/src/test/rustdoc/redirect-rename.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 File does not exist 'foo/hidden/struct.Foo.html'
     // @has foo/hidden/struct.Foo.html
5: @has check failed
 File does not exist 'foo/hidden/struct.Foo.html'
     // @has - '//p/a' '../../foo/struct.FooBar.html'
12: @has check failed
 File does not exist 'foo/hidden/bar/index.html'
     // @has foo/hidden/bar/index.html
13: @has check failed
 File does not exist 'foo/hidden/bar/index.html'
     // @has - '//p/a' '../../foo/baz/index.html'
15: @has check failed
 File does not exist 'foo/hidden/bar/struct.Thing.html'
         // @has foo/hidden/bar/struct.Thing.html
16: @has check failed
 File does not exist 'foo/hidden/bar/struct.Thing.html'
         // @has - '//p/a' '../../foo/baz/struct.Thing.html'
21: @has check failed
 File does not exist 'foo/struct.FooBar.html'
 // @has foo/struct.FooBar.html
23: @has check failed
 File does not exist 'foo/union.FooU.html'
 // @has foo/union.FooU.html
25: @has check failed
 File does not exist 'foo/enum.FooEmpty.html'
 // @has foo/enum.FooEmpty.html
27: @has check failed
 File does not exist 'foo/constant.FooC.html'
 // @has foo/constant.FooC.html
29: @has check failed
 File does not exist 'foo/static.FooS.html'
 // @has foo/static.FooS.html
32: @has check failed
 File does not exist 'foo/baz/index.html'
 // @has foo/baz/index.html
33: @has check failed
 File does not exist 'foo/baz/struct.Thing.html'
 // @has foo/baz/struct.Thing.html
Encountered 13 errors

------------------------------------------



---- [rustdoc] rustdoc/remove-duplicates.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/remove-duplicates" "/checkout/src/test/rustdoc/remove-duplicates.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @count check failed
 Expected 1 occurrences but found 0
 // @count foo/index.html '//*[@class="trait"]' 1
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/search-index.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/search-index" "/checkout/src/test/rustdoc/search-index.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `PATTERN` did not match
 // @has search-index.js Foo
11: @has check failed
 `PATTERN` did not match
         pub fn test_method() {} // @has - test_method
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/redirect.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect" "/checkout/src/test/rustdoc/redirect.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
22: @has check failed
 File does not exist 'reexp_stripped/private/struct.Quz.html'
 // @has reexp_stripped/private/struct.Quz.html
23: @has check failed
 File does not exist 'reexp_stripped/private/struct.Quz.html'
 // @has - '//p/a' '../../reexp_stripped/struct.Quz.html'
24: @has check failed
 File does not exist 'reexp_stripped/struct.Quz.html'
 // @has 'reexp_stripped/struct.Quz.html'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/synthetic_auto/complex.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/complex" "/checkout/src/test/rustdoc/synthetic_auto/complex.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
22: @has check failed
 File does not exist 'complex/struct.NotOuter.html'
 // @has complex/struct.NotOuter.html
23: @has check failed
 File does not exist 'complex/struct.NotOuter.html'
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'a, T, K: ?Sized> Send for Outer<'a, T, K> where K: for<'b> Fn((&'b bool, &'a u8)) -> &'b i8, T: MyTrait<'a>, <T as MyTrait<'a>>::MyItem: Copy, 'a: 'static"
Encountered 2 errors

------------------------------------------

---
test result: FAILED. 441 passed; 14 failed; 3 ignored; 0 measured; 0 filtered out; finished in 32.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:05
