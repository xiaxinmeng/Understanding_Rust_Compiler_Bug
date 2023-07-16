plain
.................................................................................................... 9400/11819
.................................................................................................... 9500/11819
.................................................................................................... 9600/11819
........................................i........i.................................................. 9700/11819
......................................................................................iiiiiii..iiiii 9800/11819
.................................................................................................... 10000/11819
.................................................................................................... 10100/11819
.................................................................................................... 10200/11819
.................................................................................................... 10300/11819
---
 finished in 0.432 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.180 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.39s

 finished in 2.448 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
........................................
failures:

---- [rustdoc] rustdoc/intra-doc/field.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/field" "/checkout/src/test/rustdoc/intra-doc/field.rs"
------------------------------------------
------------------------------------------
Failed to get path ".//a[@href=""https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html#structfield.start"]"
------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 263, in iterfind
    selector = _cache[cache_key]
KeyError: ('.//a[@href=""https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html#structfield.start"]', None)

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 80, in xpath_tokenizer
    raise KeyError
KeyError

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
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 155, in prepare_predicate
    token = next()
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 83, in xpath_tokenizer
    raise SyntaxError("prefix %r not found in prefix map" % prefix)
SyntaxError: prefix 'https' not found in prefix map
------------------------------------------




failures:
    [rustdoc] rustdoc/intra-doc/field.rs

test result: FAILED. 436 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 36.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:55
