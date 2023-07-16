plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 470 tests
i.......................................F....i....................................................F. 100/470
...............................................F..F..F.............................................. 200/470
.................................................................................................... 300/470
.............................................................F............Fi..F...............F..... 400/470
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] rustdoc/check-source-code-urls-to-def.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/check-source-code-urls-to-def" "/checkout/src/test/rustdoc/check-source-code-urls-to-def.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
48: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../../foo/primitive.bool.html"]' 'bool'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/hidden-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
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



---- [rustdoc] rustdoc/intra-doc-crate/self.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc-crate/self" "/checkout/src/test/rustdoc/intra-doc-crate/self.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has self/struct.S.html '//a[@href="struct.S.html#method.f"]' "Self::f"
7: @has check failed
 `XPATH PATTERN` did not match
 // @has self/struct.S.html '//a[@href="struct.S.html"]' "Self"
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/cross-crate/submodule-outer.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/submodule-outer" "/checkout/src/test/rustdoc/intra-doc/cross-crate/submodule-outer.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has 'submodule_outer/trait.Foo.html' '//a[@href="bar/trait.Bar.html"]' 'Bar'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has 'submodule_outer/trait.Foo.html' '//a[@href="trait.Baz.html"]' 'Baz'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/cross-crate/additional_doc.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/additional_doc" "/checkout/src/test/rustdoc/intra-doc/cross-crate/additional_doc.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'additional_doc/trait.Rng.html' '//a[@href="trait.Rng.html"]' 'Rng'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/proc-macro.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/proc-macro" "/checkout/src/test/rustdoc/proc-macro.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
70: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//a/@href' '../derive.SomeDerive.html'
Encountered 1 errors

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
8: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//p/a' '../../foo/static.STATIC_FOO.html'
11: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//p/a' '../../foo/constant.CONST_FOO.html'
Encountered 2 errors

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
5: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//p/a' '../../foo/struct.FooBar.html'
13: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//p/a' '../../foo/baz/index.html'
16: @has check failed
 `XPATH PATTERN` did not match
         // @has - '//p/a' '../../foo/baz/struct.Thing.html'
Encountered 3 errors

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
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//p/a' '../../reexp_stripped/struct.Bar.html'
23: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//p/a' '../../reexp_stripped/struct.Quz.html'
Encountered 2 errors

------------------------------------------

---
test result: FAILED. 458 passed; 9 failed; 3 ignored; 0 measured; 0 filtered out; finished in 41.13s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:58
