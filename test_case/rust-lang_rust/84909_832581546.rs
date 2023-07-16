plain
.................................................................................................... 9400/11810
.................................................................................................... 9500/11810
.................................................................................................... 9600/11810
................................i.......i........................................................... 9700/11810
..............................................................................iiiiiii..iiiiii.i..... 9800/11810
.................................................................................................... 10000/11810
.................................................................................................... 10100/11810
.................................................................................................... 10200/11810
.................................................................................................... 10300/11810
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.166 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.42s

 finished in 2.477 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
warning: `tidy` is not installed; diffs will not be generated

running 436 tests
i......................................i............................................................ 100/436
.....................................................FF.....F.FFFF.FF.......F....................... 200/436
....................................F............i.................................................. 400/436
....................................
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:
failures:

---- [rustdoc] rustdoc/intra-doc/prim-methods-external-core.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods-external-core" "/checkout/src/test/rustdoc/intra-doc/prim-methods-external-core.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.char.html"]' 'char'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.char.html#method.len_utf8"]' 'char::len_utf8'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/prim-methods-local.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods-local" "/checkout/src/test/rustdoc/intra-doc/prim-methods-local.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.char.html"]' 'char'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.char.html#method.len_utf8"]' 'char::len_utf8'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/non-path-primitives.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/non-path-primitives" "/checkout/src/test/rustdoc/intra-doc/non-path-primitives.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.rotate_left"]' 'slice::rotate_left'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.array.html#method.map"]' 'array::map'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html"]' 'owned str'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html"]' 'str ref'
13: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_empty"]' 'str::is_empty'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html#method.len"]' '&str::len'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html#method.is_null"]' 'pointer::is_null'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html#method.is_null"]' '*const::is_null'
22: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html#method.is_null"]' '*mut::is_null'
27: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.unit.html"]' 'unit'
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html"]' 'tuple'
33: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.reference.html"]' 'reference'
34: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.reference.html"]' '&'
35: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.reference.html"]' '&mut'
40: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.fn.html"]' 'fn'
43: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.never.html"]' 'never'
44: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.never.html"]' '!'
Encountered 17 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/prim-assoc.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-assoc" "/checkout/src/test/rustdoc/intra-doc/prim-assoc.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has prim_assoc/index.html '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX"]' "i32::MAX"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/prim-methods.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods" "/checkout/src/test/rustdoc/intra-doc/prim-methods.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.char.html"]' 'char'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.char.html#method.len_utf8"]' 'char::len_utf8'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/prim-precedence.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-precedence" "/checkout/src/test/rustdoc/intra-doc/prim-precedence.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 `XPATH PATTERN` did not match
     // @has prim_precedence/char/struct.Inner.html '//a/@href' 'https://doc.rust-lang.org/nightly/std/primitive.char.html'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has prim_precedence/struct.MyString.html '//a/@href' 'https://doc.rust-lang.org/nightly/std/primitive.char.html'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/primitive-disambiguators.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/primitive-disambiguators" "/checkout/src/test/rustdoc/intra-doc/primitive-disambiguators.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' 'https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-link-prim-self.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-prim-self" "/checkout/src/test/rustdoc/intra-link-prim-self.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.usize.html#method.f"]' 'Self::f'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="https://doc.rust-lang.org/nightly/std/primitive.usize.html#associatedconstant.MAX"]' 'Self::MAX'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/primitive-non-default-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/primitive-non-default-impl" "/checkout/src/test/rustdoc/intra-doc/primitive-non-default-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim"]' 'str::trim'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html#method.to_lowercase"]' 'str::to_lowercase'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html#method.into_boxed_bytes"]' 'str::into_boxed_bytes'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html#method.replace"]' 'str::replace'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.powi"]' 'f32::powi'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.sqrt"]' 'f32::sqrt'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.mul_add"]' 'f32::mul_add'
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.powi"]' 'f64::powi'
28: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.sqrt"]' 'f64::sqrt'
30: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.mul_add"]' 'f64::mul_add'
Encountered 10 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/true-false.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/true-false" "/checkout/src/test/rustdoc/intra-doc/true-false.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.bool.html"]' 'true'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]//a[@href="https://doc.rust-lang.org/nightly/std/primitive.bool.html"]' 'false'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/primitive-link.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-link" "/checkout/src/test/rustdoc/primitive-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//*[@class="docblock"]/p/a[@href="https://doc.rust-lang.org/nightly/std/primitive.u32.html"]' 'u32'
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//*[@class="docblock"]/p/a[@href="https://doc.rust-lang.org/nightly/std/primitive.i64.html"]' 'i64'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//*[@class="docblock"]/p/a[@href="https://doc.rust-lang.org/nightly/std/primitive.i32.html"]' 'std::primitive::i32'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//*[@class="docblock"]/p/a[@href="https://doc.rust-lang.org/nightly/std/primitive.str.html"]' 'std::primitive::str'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//*[@class="docblock"]/p/a[@href="https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX"]' 'std::primitive::i32::MAX'
Encountered 5 errors

------------------------------------------

---
test result: FAILED. 422 passed; 11 failed; 3 ignored; 0 measured; 0 filtered out; finished in 36.34s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:44
