plain
.................................................................................................... 9300/11704
.................................................................................................... 9400/11704
.................................................................................................... 9500/11704
.............................................i......i............................................... 9600/11704
...........................................................................................iiiiiii.. 9700/11704
iiiiii.i............................................................................................ 9800/11704
.................................................................................................... 10000/11704
.................................................................................................... 10100/11704
.................................................................................................... 10200/11704
.................................................................................................... 10300/11704
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.099 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.37s

 finished in 2.438 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 429 tests
i.F...................F.................iF.................................................F....F... 100/429
..................................FFFF.FFFF..FF....F......FF...F....FF.F..F.FF.......F..F........... 200/429
......................F............................................................F...........F.... 300/429
......F...............................F......iF...F.F................F..........F................... 400/429
......F......................
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/assoc-types.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="tymethod.index"]//code//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' "Output"
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' 'Output'
29: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Feed.html#associatedtype.Input"]' 'Input'
34: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Feed.html#associatedtype.Input"]' 'Input'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/check-styled-link.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-styled-link" "/checkout/src/test/rustdoc/check-styled-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Bar.html '//a[@href="../foo/struct.Foo.html"]' 'Foo'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/default-trait-method-link.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method-link" "/checkout/src/test/rustdoc/default-trait-method-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/trait.Foo.html '//a[@href="../foo/trait.Foo.html#tymethod.req"]' 'req'
4: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/trait.Foo.html '//a[@href="../foo/trait.Foo.html#method.prov"]' 'prov'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/hidden-impls.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls" "/checkout/src/test/rustdoc/hidden-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @has check failed
 File does not exist 'implementors/foo/trait.Clone.js'
 // @has implementors/foo/trait.Clone.js
16: @!has check failed
 File does not exist 'implementors/foo/trait.Clone.js'
 // @!has - 'Foo'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/impl-disambiguation.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-disambiguation" "/checkout/src/test/rustdoc/impl-disambiguation.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
25: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl Foo for foo::mod1::Baz"
28: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/trait.Foo.html '//*[@class="item-list"]//code' "impl<'a> Foo for &'a foo::mod2::Baz"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/anchors.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/anchors" "/checkout/src/test/rustdoc/intra-doc/anchors.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../anchors/struct.Something.html#Anchor!'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/associated-defaults.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/associated-defaults" "/checkout/src/test/rustdoc/intra-doc/associated-defaults.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_defaults/struct.UsesDefaults.html' '//a[@href="../associated_defaults/struct.UsesDefaults.html#associatedtype.T"]' 'UsesDefaults::T'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_defaults/struct.UsesDefaults.html' '//a[@href="../associated_defaults/struct.UsesDefaults.html#method.f"]' 'UsesDefaults::f'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_defaults/struct.OverridesDefaults.html' '//a[@href="../associated_defaults/struct.OverridesDefaults.html#associatedtype.T"]' 'OverridesDefaults::T'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_defaults/struct.OverridesDefaults.html' '//a[@href="../associated_defaults/struct.OverridesDefaults.html#method.f"]' 'OverridesDefaults::f'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/associated-items.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/associated-items" "/checkout/src/test/rustdoc/intra-doc/associated-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/struct.MyStruct.html' '//a[@href="../associated_items/struct.MyStruct.html"]' 'MyStruct'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/struct.MyStruct.html' '//a[@href="../associated_items/struct.MyStruct.html#method.method"]' 'link from struct'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/struct.MyStruct.html' '//a[@href="../associated_items/struct.MyStruct.html#method.clone"]' 'MyStruct::clone'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/struct.MyStruct.html' '//a[@href="../associated_items/struct.MyStruct.html#associatedtype.Input"]' 'MyStruct::Input'
34: @has check failed
 `XPATH PATTERN` did not match
     // @has 'associated_items/struct.MyStruct.html' '//a[@href="../associated_items/struct.MyStruct.html#method.method"]' 'link from method'
Encountered 5 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/basic.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/basic" "/checkout/src/test/rustdoc/intra-doc/basic.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
2: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/struct.ThisType.html'
3: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/struct.ThisType.html#method.this_method'
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/enum.ThisEnum.html'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/enum.ThisEnum.html#variant.ThisVariant'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/trait.ThisTrait.html'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/trait.ThisTrait.html#tymethod.this_associated_method'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/trait.ThisTrait.html#associatedtype.ThisAssociatedType'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/trait.ThisTrait.html#associatedconstant.THIS_ASSOCIATED_CONST'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/trait.ThisTrait.html'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/type.ThisAlias.html'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/union.ThisUnion.html'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/fn.this_function.html'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/constant.THIS_CONST.html'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/static.THIS_STATIC.html'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/macro.this_macro.html'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/trait.SoAmbiguous.html'
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/fn.SoAmbiguous.html'
49: @has check failed
 `XPATH PATTERN` did not match
 // @has basic/struct.ThisType.html '//a/@href' '../basic/macro.this_macro.html'
75: @has check failed
 `XPATH PATTERN` did not match
 // @has basic/struct.SomeOtherType.html '//a/@href' '../basic/struct.ThisType.html'
76: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/struct.ThisType.html#method.this_method'
77: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/enum.ThisEnum.html'
78: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/enum.ThisEnum.html#variant.ThisVariant'
Encountered 22 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/cross-crate/submodule-outer.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/submodule-outer" "/checkout/src/test/rustdoc/intra-doc/cross-crate/submodule-outer.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has 'submodule_outer/trait.Foo.html' '//a[@href="../submodule_outer/bar/trait.Bar.html"]' 'Bar'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has 'submodule_outer/trait.Foo.html' '//a[@href="../submodule_outer/trait.Baz.html"]' 'Baz'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc-crate/self.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc-crate/self" "/checkout/src/test/rustdoc/intra-doc-crate/self.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has self/struct.S.html '//a[@href="../self/struct.S.html#method.f"]' "Self::f"
7: @has check failed
 `XPATH PATTERN` did not match
 // @has self/struct.S.html '//a[@href="../self/struct.S.html"]' "Self"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/disambiguators-removed.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/disambiguators-removed" "/checkout/src/test/rustdoc/intra-doc/disambiguators-removed.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/trait.Name.html"][code]' "Name"
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"][code]' "Name"
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/macro.Name.html"][code]' "Name"
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"][code]' "Name()"
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/macro.Name.html"][code]' "Name!"
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"]' "Name"
25: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/macro.Name.html"]' "Name!"
33: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/macro.Name.html"]' "Name"
37: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/trait.Name.html"]' "../disambiguators_removed/macro.Name.html"
44: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"]' "fn@Name"
48: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/trait.Name.html"]' "trait@Name"
Encountered 11 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/enum-struct-field.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/enum-struct-field" "/checkout/src/test/rustdoc/intra-doc/enum-struct-field.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/fn.foo.html '//a/@href' '../foo/enum.Foo.html#variant.X.field.y'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/extern-type.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/extern-type" "/checkout/src/test/rustdoc/intra-doc/extern-type.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `PATTERN` did not match
 // @has 'extern_type/fn.links_to_extern_type.html' 'href="../extern_type/foreigntype.ExternType.html#method.f"'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/cross-crate/additional_doc.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/additional_doc" "/checkout/src/test/rustdoc/intra-doc/cross-crate/additional_doc.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'additional_doc/trait.Rng.html' '//a[@href="../additional_doc/trait.Rng.html"]' 'Rng'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/cross-crate/hidden.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/hidden" "/checkout/src/test/rustdoc/intra-doc/cross-crate/hidden.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has 'hidden/struct.Ready.html' '//a/@href' '../hidden/fn.ready.html'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/issue-82209.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/issue-82209" "/checkout/src/test/rustdoc/intra-doc/issue-82209.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/enum.Foo.html '//a/@href' '../foo/enum.Foo.html#variant.Bar.field.abc'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/mod-ambiguity.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/mod-ambiguity" "/checkout/src/test/rustdoc/intra-doc/mod-ambiguity.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
---
test result: FAILED. 388 passed; 38 failed; 3 ignored; 0 measured; 0 filtered out; finished in 35.45s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:20
