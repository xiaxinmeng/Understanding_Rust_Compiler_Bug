plain
warning: `tidy` is not installed; diffs will not be generated

running 444 tests
i........................................i.......................................................... 100/444
......................................F....................F.............F...F...................... 200/444
.......................................F..........i................................................. 400/444
............................................
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:
failures:

---- [rustdoc] rustdoc/intra-doc/associated-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/associated-items" "/checkout/src/test/rustdoc/intra-doc/associated-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/fn.foo.html' '//a[@href="{{channel}}/alloc/collections/btree/map/struct.BTreeMap.html#method.into_iter"]' 'std::collections::BTreeMap::into_iter'
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/fn.foo.html' '//a[@href="{{channel}}/alloc/string/struct.String.html#method.from"]' 'String::from'
8: @has check failed
 `XPATH PATTERN` did not match
 // @has 'associated_items/fn.foo.html' '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.into_iter"]' 'Vec::into_iter'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/generic-params.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/generic-params" "/checkout/src/test/rustdoc/intra-doc/generic-params.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html"]' 'Vec<T>'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/boxed/struct.Box.html"]' 'Box<Vec<Option<T>>>'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item"]' 'Iterator<Box<T>>::Item'
14: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/option/enum.Option.html"]' 'just Option'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/option/enum.Option.html"]' 'with the generic, Option<T>'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/result/enum.Result.html"]' 'Result<T, E>'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/result/enum.Result.html"]' 'Result<T, !>'
22: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/result/enum.Result.html"]' 'Result<!, E>'
28: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.new"]' 'Vec::<T>::new'
29: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.new"]' 'with parentheses as Vec::<T>::new()'
30: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.new"]' 'Vec::<Box<T>>::new()'
35: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/core/any/struct.TypeId.html#method.of"]' 'TypeId::of::<String>()'
36: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/vec/struct.Vec.html#method.len"]' 'Vec::<std::error::Error>::len'
41: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/index.html '//a[@href="{{channel}}/alloc/boxed/struct.Box.html#method.new"]' 'Box::<T>new()'
Encountered 14 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/pub-use.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/pub-use" "/checkout/src/test/rustdoc/intra-doc/pub-use.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="{{channel}}/std/env/index.html"]' "std::env"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/intra-doc/trait-item.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/trait-item" "/checkout/src/test/rustdoc/intra-doc/trait-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@href="{{channel}}/core/default/trait.Default.html#tymethod.default"]' 'Default::default()'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/primitive-reexport.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-reexport" "/checkout/src/test/rustdoc/primitive-reexport.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code/a[@href="{{channel}}/std/primitive.bool.html"]' 'bool'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code/a[@href="{{channel}}/std/primitive.char.html"]' 'char'
17: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code/a[@href="{{channel}}/std/primitive.bool.html"]' 'bool'
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code/a[@href="{{channel}}/std/primitive.char.html"]' 'char'
24: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code/a[@href="{{channel}}/std/primitive.str.html"]' 'str'
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//code/a[@href="{{channel}}/std/primitive.i32.html"]' 'i32'
Encountered 6 errors

------------------------------------------

---
test result: FAILED. 436 passed; 5 failed; 3 ignored; 0 measured; 0 filtered out; finished in 25.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:21
