plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 483 tests
i.F............F................F...................i.F.........................F......F.F.......... 100/483
.....F.......................F..F.F.FF...F.......................................................... 200/483
...................F....F........................................................................FF. 300/483
.........F.F...........i..F.F.F......................................F..................i........... 400/483
................F.........F.F.FFF.F...FF...........................................
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.S.html '//div[@id="impl-Into%3CU%3E"]//h3[@class="code-header in-band"]' 'impl<T, U> Into<U> for T'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/primitive.i16.html' '//h2[@id="synthetic-implementations"]' 'Auto Trait Implementation'
Encountered 1 errors

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
12: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Send"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Send for VSet<T, ORDER>'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Sync"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Sync for VSet<T, ORDER>'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-recursive-pathbuf.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf" "/checkout/src/test/rustdoc/deref-recursive-pathbuf.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@id="deref-methods-PathBuf"]' 'Methods from Deref<Target = PathBuf>'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.as_path"]' 'pub fn as_path(&self)'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@id="deref-methods-Path"]' 'Methods from Deref<Target = Path>'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="impl-items"]//*[@id="method.exists"]' 'pub fn exists(&self)'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-PathBuf"]' 'Methods from Deref<Target=PathBuf>'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.as_path"]' 'as_path'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-Path"]' 'Methods from Deref<Target=Path>'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="sidebar-links"]/a[@href="#method.exists"]' 'exists'
Encountered 8 errors

------------------------------------------



---- [rustdoc] rustdoc/empty-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-impls" "/checkout/src/test/rustdoc/empty-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="synthetic-implementations-list"]/div[@id="impl-Send"]' 'impl Send for Foo'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/escape-deref-methods.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods" "/checkout/src/test/rustdoc/escape-deref-methods.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-title"]' 'Methods from Deref<Target=Vec<Title>>'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/extern-default-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method" "/checkout/src/test/rustdoc/extern-default-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @count check failed
 Expected 1 occurrences but found 0
 // @count extern_default_method/struct.Struct.html '//*[@id="method.provided"]' 1
7: @has check failed
 `XPATH PATTERN` did not match
 // @has extern_default_method/struct.Struct.html '//div[@id="method.provided"]//a[@class="fnname"]/@href' #method.provided
8: @has check failed
 `XPATH PATTERN` did not match
 // @has extern_default_method/struct.Struct.html '//div[@id="method.provided"]//a[@class="anchor"]/@href' #method.provided
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/generic-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//div[@id="impl-ToString"]//h3[@class="code-header in-band"]' 'impl<T> ToString for T'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//div[@class="sidebar-links"]/a[@href="#impl-ToString"]' 'ToString'
Encountered 2 errors

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
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.my_trait_method"]' 'fn my_trait_method()'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@class="docblock"]' 'docs for my_trait_method'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/issue-31948-1.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-1" "/checkout/src/test/rustdoc/inline_cross/issue-31948-1.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'for Foo'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'for Foo'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/issue-31948-2.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-31948-2" "/checkout/src/test/rustdoc/inline_cross/issue-31948-2.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' 'Bark for'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' 'Woof for'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'for Foo'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/inline_cross/issue-33113.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-33113" "/checkout/src/test/rustdoc/inline_cross/issue-33113.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' "for Foo"
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
 // @has - '//h3[@class="code-header in-band"]' "impl<'a> Debug for dyn Bar"
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
 // @has - '//h3[@class="code-header in-band"]' 'impl Clone for SomeStruct'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-19190-2.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-2" "/checkout/src/test/rustdoc/issue-19190-2.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.as_str"]' 'fn as_str(&self) -> &str'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3" "/checkout/src/test/rustdoc/issue-19190-3.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.as_str"]' 'fn as_str(&self) -> &str'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.baz"]' 'fn baz(&self)'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="method.baz"]' 'fn baz(&self)'
Encountered 3 errors

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
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'impl<B> Send for Switch<B> where <B as Signal>::Item: Send'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'impl<B> Sync for Switch<B> where <B as Signal>::Item: Sync'
17: @count check failed
 Expected 5 occurrences but found 0
 // @count - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]' 5
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-51236.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-51236" "/checkout/src/test/rustdoc/issue-51236.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<T> Send for Owned<T> where <T as Owned<'static>>::Reader: Send"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-56822.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56822" "/checkout/src/test/rustdoc/issue-56822.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<'a> Send for Parser<'a>"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-60726.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-60726" "/checkout/src/test/rustdoc/issue-60726.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
31: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="synthetic-implementations-list"]//*[@class="impl has-srclink"]//h3[@class="code-header in-band"]' "impl<T> !Sync for IntoIter<T>"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-80233-normalize-auto-trait.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-80233-normalize-auto-trait" "/checkout/src/test/rustdoc/issue-80233-normalize-auto-trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
34: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="code-header in-band"]' 'impl<T> Send for Question<T>'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-75588.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-75588" "/checkout/src/test/rustdoc/issue-75588.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
---
test result: FAILED. 446 passed; 33 failed; 4 ignored; 0 measured; 0 filtered out; finished in 20.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:03
