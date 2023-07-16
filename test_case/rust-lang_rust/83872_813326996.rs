plain
.................................................................................................... 9400/11731
.................................................................................................... 9500/11731
........................................................................i......i.................... 9600/11731
.................................................................................................... 9700/11731
...................iiiiiii.iiiiii.i................................................................. 9800/11731
.................................................................................................... 10000/11731
.................................................................................................... 10100/11731
.................................................................................................... 10200/11731
.................................................................................................... 10300/11731
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.094 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.26s

 finished in 2.325 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 430 tests
i.....F......F................F...F.....i.......FF..........F..............................F........ 100/430
......................................F....F..............................F......................... 200/430
....................F..............F........................................................F....... 300/430
..............................................i....................F................................ 400/430
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..............F...............

---- [rustdoc] rustdoc/assoc-consts.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
35: @has check failed
 `XPATH PATTERN` did not match
     // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.BAZ"]' "const BAZ: Baz<'static, u8, u32>"
43: @has check failed
 `XPATH PATTERN` did not match
     // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.F"]' "const F: fn(_: &(dyn ToString + 'static))"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/async-fn.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
80: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//h4[@class="method"]' 'pub async fn complicated_lifetimes( &self, context: &impl Bar) -> impl Iterator<Item = &usize>'
83: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//h4[@class="method"]' "pub async fn readable<T>(&self) -> Result<AsyncFdReadyGuard<'_, T>, ()>"
85: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//h4[@class="method"]' "pub async fn mut_self(&mut self)"
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
25: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//h3[@id="impl-1"]/code' 'impl<T> VSet<T, {Order::Unsorted}>'
34: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Escape.html '//h3[@id="impl"]/code' 'impl Escape<{ r#"<script>alert("Escape");</script>"# }>'
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
50: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Bar.html '//h3[@id="impl"]/code' 'impl<const M: usize> Bar<u8, M>'
52: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.hey"]' 'pub fn hey<const N: usize>(&self) -> Foo<N> where u8: Trait<N>'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-typedef.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef" "/checkout/src/test/rustdoc/deref-typedef.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_b"]' 'pub fn foo_b(&self)'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_c"]' 'pub fn foo_c(&self)'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.foo_j"]' 'pub fn foo_j(&self)'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.foo_b"]' 'foo_b'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.foo_c"]' 'foo_c'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.foo_j"]' 'foo_j'
Encountered 6 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-recursive.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive" "/checkout/src/test/rustdoc/deref-recursive.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@id="deref-methods-Baz"]' 'Methods from Deref<Target = Baz>'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.baz"]' 'pub fn baz(&self)'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"][@href="#deref-methods-Baz"]' 'Methods from Deref<Target=Baz>'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.baz"]' 'baz'
Encountered 4 errors

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
82: @count check failed
 Expected 10 occurrences but found 6
     // @count   - '//*[@class="stab portability"]' 10
89: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature broadcloth'
90: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features broadcloth and xanthocomic'
91: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature broadcloth'
92: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features broadcloth and whosoever'
Encountered 5 errors

------------------------------------------



---- [rustdoc] rustdoc/glob-shadowing.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/glob-shadowing" "/checkout/src/test/rustdoc/glob-shadowing.rs"
------------------------------------------
------------------------------------------
Failed to get path ".//div[@class=docblock]"
------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 263, in iterfind
    selector = _cache[cache_key]
KeyError: ('.//div[@class=docblock]', None)

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 435, in check_command
    ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
  File "/checkout/src/etc/htmldocck.py", line 366, in check_tree_text
    for e in tree.findall(path):
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 304, in findall
    return list(iterfind(elem, path, namespaces))
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 277, in iterfind
    selector.append(ops[token[0]](next, token))
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 233, in prepare_predicate
    raise SyntaxError("invalid predicate")
SyntaxError: invalid predicate
------------------------------------------


---- [rustdoc] rustdoc/intra-doc/basic.rs stdout ----
---- [rustdoc] rustdoc/intra-doc/basic.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/basic" "/checkout/src/test/rustdoc/intra-doc/basic.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../basic/fn.SoAmbiguous.html'
Encountered 1 errors

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
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"][code]' "Name"
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"][code]' "Name()"
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"]' "Name"
31: @has check failed
 File does not exist 'disambiguators_removed/fn.Name.html'
 // @has disambiguators_removed/fn.Name.html
32: @has check failed
 File does not exist 'disambiguators_removed/fn.Name.html'
 // @has - '//a[@href="../disambiguators_removed/macro.Name.html"]' "Name"
36: @has check failed
 File does not exist 'disambiguators_removed/fn.Name.html'
 // @has - '//a[@href="../disambiguators_removed/trait.Name.html"]' "../disambiguators_removed/macro.Name.html"
43: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../disambiguators_removed/fn.Name.html"]' "fn@Name"
Encountered 7 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/self.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/self" "/checkout/src/test/rustdoc/intra-doc/self.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
83: @has check failed
 `XPATH PATTERN` did not match
     // @has foo/struct.MyStruct.html '//a/@href' '../foo/struct.MyStruct.html#method.for_impl'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-25001.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001" "/checkout/src/test/rustdoc/issue-25001.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.pass-1"]//code' 'fn pass() -> usize'
19: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="method.pass-2"]//code' 'fn pass() -> isize'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-32890.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32890" "/checkout/src/test/rustdoc/issue-32890.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//a[@href="#method.pass-1"]' 'pass'
15: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//a[@href="#method.pass-2"]' 'pass'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-61592.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-61592" "/checkout/src/test/rustdoc/issue-61592.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code' 'pub use foo::FooStruct as _;'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/sidebar-link-generation.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-link-generation" "/checkout/src/test/rustdoc/sidebar-link-generation.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.SomeStruct.html '//*[@class="sidebar-links"]/a[@href="#method.some_fn-1"]' "some_fn"
Encountered 1 errors

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
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl"]//code' 'impl MyAlias'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar"]//a[@href="#implementations"]' 'Methods'
Encountered 2 errors

------------------------------------------

---
test result: FAILED. 411 passed; 16 failed; 3 ignored; 0 measured; 0 filtered out; finished in 35.73s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:16
