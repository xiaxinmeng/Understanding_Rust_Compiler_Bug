plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 486 tests
i.F...............................FF.................i.......F............F......................... 100/486
......................F....................................F........................................ 200/486
....F............................................................................................... 300/486
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F.F..................................................F...F...........F.............F..

---- [rustdoc] rustdoc/assoc-item-cast.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast" "/checkout/src/test/rustdoc/assoc-item-cast.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 File does not exist 'foo/type.AsExprOf.html'
 // @has foo/type.AsExprOf.html
14: @has check failed
 File does not exist 'foo/type.AsExprOf.html'
 // @has - '//*[@class="rust typedef"]' 'type AsExprOf<Item, Type> = <Item as AsExpression<Type>>::Expression;'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/const-generics/const-generics-docs.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 File does not exist 'foo/type.TyAlias.html'
 // @has foo/type.TyAlias.html '//pre[@class="rust typedef"]' 'type TyAlias<const N: usize> = ExternTy<N>;'
81: @has check failed
 File does not exist 'foo/type.Faz.html'
 // @has foo/type.Faz.html '//pre[@class="rust typedef"]' 'type Faz<const N: usize> = [u8; N];'
84: @has check failed
 File does not exist 'foo/type.Fiz.html'
 // @has foo/type.Fiz.html '//pre[@class="rust typedef"]' 'type Fiz<const N: usize> = [[u8; N]; 48];'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/const-generics/type-alias.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/type-alias" "/checkout/src/test/rustdoc/const-generics/type-alias.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'foo/type.CellIndex.html'
 // @has foo/type.CellIndex.html '//pre[@class="rust typedef"]' 'type CellIndex<const D: usize> = [i64; D];'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/cross-crate-links.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-links" "/checkout/src/test/rustdoc/cross-crate-links.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
45: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/index.html' '//a[@href="../all_item_types/type.FooType.html"]' 'FooType'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/doc-cfg-simplification.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification" "/checkout/src/test/rustdoc/doc-cfg-simplification.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
53: @has check failed
 File does not exist 'globuliferous/ratel/type.Wrick.html'
     // @has 'globuliferous/ratel/type.Wrick.html'
54: @count check failed
 File does not exist 'globuliferous/ratel/type.Wrick.html'
     // @count   - '//*[@class="stab portability"]' 1
55: @matches check failed
 File does not exist 'globuliferous/ratel/type.Wrick.html'
     // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
58: @has check failed
 File does not exist 'globuliferous/ratel/type.Thionic.html'
     // @has 'globuliferous/ratel/type.Thionic.html'
59: @count check failed
 File does not exist 'globuliferous/ratel/type.Thionic.html'
     // @count   - '//*[@class="stab portability"]' 1
60: @matches check failed
 File does not exist 'globuliferous/ratel/type.Thionic.html'
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and thionic'
Encountered 6 errors

------------------------------------------



---- [rustdoc] rustdoc/impl-trait-alias.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/impl-trait-alias" "/checkout/src/test/rustdoc/impl-trait-alias.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 File does not exist 'impl_trait_alias/type.Foo.html'
 // @has impl_trait_alias/type.Foo.html 'Foo'
Encountered 1 errors

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
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' 'type.ThisAlias.html'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/type-alias.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/type-alias" "/checkout/src/test/rustdoc/intra-doc/type-alias.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
18: @has check failed
 File does not exist 'foo/type.Bar.html'
 // @has foo/type.Bar.html '//a[@href="struct.Foo.html#method.bar"]' 'Self::bar'
19: @has check failed
 File does not exist 'foo/type.Int.html'
 // @has foo/type.Int.html '//a[@href="{{channel}}/std/primitive.i32.html#associatedconstant.MIN"]' 'Self::MIN'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/reexports-priv.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/reexports-priv" "/checkout/src/test/rustdoc/reexports-priv.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
44: @has check failed
 File does not exist 'foo/type.Type.html'
 // @has 'foo/type.Type.html' '//*[@class="rust typedef"]' 'pub type Type ='
108: @has check failed
 File does not exist 'foo/outer/inner/type.Type.html'
         // @has 'foo/outer/inner/type.Type.html' '//*[@class="rust typedef"]' 'pub type Type ='
110: @has check failed
 File does not exist 'foo/outer/inner/type.TypeCrate.html'
         // @has 'foo/outer/inner/type.TypeCrate.html' '//*[@class="rust typedef"]' 'pub(crate) type TypeCrate ='
112: @has check failed
 File does not exist 'foo/outer/inner/type.TypeSuper.html'
         // @has 'foo/outer/inner/type.TypeSuper.html' '//*[@class="rust typedef"]' 'pub(in outer) type TypeSuper ='
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/reexports.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/reexports" "/checkout/src/test/rustdoc/reexports.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
43: @has check failed
 File does not exist 'foo/type.Type.html'
 // @has 'foo/type.Type.html' '//*[@class="rust typedef"]' 'pub type Type ='
107: @has check failed
 File does not exist 'foo/outer/inner/type.Type.html'
         // @has 'foo/outer/inner/type.Type.html' '//*[@class="rust typedef"]' 'pub type Type ='
Encountered 2 errors

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
29: @matches check failed
 File does not exist 'foo/type.FooType.html'
 // @matches 'foo/type.FooType.html' '//h1' 'Type Definition foo::FooType'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/trait-impl-items-links-and-anchors.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-impl-items-links-and-anchors" "/checkout/src/test/rustdoc/trait-impl-items-links-and-anchors.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
     // @has trait_impl_items_links_and_anchors/trait.MyTrait.html '//div[@id="associatedtype.Assoc-1"]//a[@class="type"]/@href' #associatedtype.Assoc
26: @has check failed
 `XPATH PATTERN` did not match
     // @has trait_impl_items_links_and_anchors/trait.MyTrait.html '//div[@id="associatedtype.Assoc-2"]//a[@class="type"]/@href' #associatedtype.Assoc
42: @has check failed
 `XPATH PATTERN` did not match
     // @has trait_impl_items_links_and_anchors/struct.MyStruct.html '//div[@id="associatedtype.Assoc"]//a[@class="type"]/@href' trait.MyTrait.html#associatedtype.Assoc
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/typedef.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/typedef" "/checkout/src/test/rustdoc/typedef.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 File does not exist 'typedef/type.MyAlias.html'
 // @has typedef/type.MyAlias.html
12: @has check failed
 File does not exist 'typedef/type.MyAlias.html'
 // @has - '//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' 'impl MyAlias'
13: @has check failed
 File does not exist 'typedef/type.MyAlias.html'
 // @has - '//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' 'impl MyTrait for MyAlias'
14: @has check failed
 File does not exist 'typedef/type.MyAlias.html'
 // @has - 'Alias docstring'
15: @has check failed
 File does not exist 'typedef/type.MyAlias.html'
 // @has - '//*[@class="sidebar"]//*[@class="location"]' 'Type Definition MyAlias'
16: @has check failed
 File does not exist 'typedef/type.MyAlias.html'
 // @has - '//*[@class="sidebar"]//a[@href="#implementations"]' 'Methods'
17: @has check failed
 File does not exist 'typedef/type.MyAlias.html'
 // @has - '//*[@class="sidebar"]//a[@href="#trait-implementations"]' 'Trait Implementations'
Encountered 7 errors

------------------------------------------



---- [rustdoc] rustdoc/where.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where" "/checkout/src/test/rustdoc/where.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
36: @has check failed
 File does not exist 'foo/type.Golf.html'
 // @has foo/type.Golf.html '//pre[@class="rust typedef"]' "type Golf<T> where T: Clone, = (T, T)"
Encountered 1 errors

------------------------------------------

---
test result: FAILED. 468 passed; 14 failed; 4 ignored; 0 measured; 0 filtered out; finished in 49.61s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:16:43
