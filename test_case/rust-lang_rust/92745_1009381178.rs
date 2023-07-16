plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 491 tests
i..........F.....................F...................i..............................F............... 100/491
..........F.....F................................................................................... 200/491
......................................................F............................................. 300/491
.............................i...................................................F.............i.... 400/491
...................F.....F....................................F............................
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



---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Send"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Send for VSet<T, ORDER>'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-Sync"]/h3[@class="code-header in-band"]' 'impl<T, const ORDER: Order> Sync for VSet<T, ORDER>'
Encountered 2 errors

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
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="trait-implementations-list"]/div[@id="impl-EmptyTrait"]' 'impl EmptyTrait for Foo'
16: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="trait-implementations-list"]/details/summary/div[@id="impl-NotEmpty"]' 'impl NotEmpty for Foo'
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
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/hidden-trait-struct-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-struct-impls" "/checkout/src/test/rustdoc/hidden-trait-struct-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
18: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Bar.html '//*[@id="impl-Bam"]' 'impl Bam for Bar'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-29503.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - "//div[@id='implementors-list']//div[@id='impl-MyTrait']//h3[@class='code-header in-band']" "impl<T> MyTrait for T where T: Debug"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/primitive/primitive-generic-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive/primitive-generic-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/primitive.i32.html '//div[@id="impl-ToString"]//h3[@class="code-header in-band"]' 'impl<T> ToString for T'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/sized_trait.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sized_trait" "/checkout/src/test/rustdoc/sized_trait.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Sized"]//h3[@class="code-header in-band"]' 'impl !Sized for Unsized'
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
4: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Sized"]/h3[@class="code-header in-band"]' 'impl !Sized for Unsized'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Sync"]/h3[@class="code-header in-band"]' 'impl Sync for Unsized'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Any"]/h3[@class="code-header in-band"]' 'impl<T> Any for T'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@id="impl-Any"]//a[@class="srclink"]' 'source'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/trait-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-impl" "/checkout/src/test/rustdoc/trait-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
46: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="impl-Trait"]/h3//a/@href' 'trait.Trait.html'
Encountered 1 errors

------------------------------------------

---
test result: FAILED. 477 passed; 10 failed; 4 ignored; 0 measured; 0 filtered out; finished in 33.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:03
