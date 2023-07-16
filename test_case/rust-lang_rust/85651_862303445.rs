plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 445 tests
i........................................i....................F....F................................ 100/445
........................................................................................F........... 300/445
........................................................................................F........... 300/445
....................................................i.......F......F................................ 400/445
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] rustdoc/doc-cfg.rs stdout ----
---- [rustdoc] rustdoc/doc-cfg.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
15: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="module-item"]//*[@class="stab portability"]' '\AARM\Z'
45: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="module-item"]//*[@class="stab portability"]' '\AWebAssembly\Z'
77: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="module-item"]//*[@class="stab portability"]' '\Aavx\Z'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/duplicate-cfg.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate-cfg" "/checkout/src/test/rustdoc/duplicate-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @matches check failed
 `XPATH PATTERN` did not match
 // @matches '-' '//*[@class="module-item"]//*[@class="stab portability"]' '^sync$'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="module-item"]//*[@class="stab portability"]/@title' 'This is supported on crate feature `sync` only'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-55364.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-55364" "/checkout/src/test/rustdoc/issue-55364.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
32: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//section[@id="main"]/table//tr[@class="module-item"]/td/a[@class="fn"][@href="fn.foo.html"]' 'foo'
33: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//section[@id="main"]/table//tr[@class="module-item"]/td/a[@class="fn"][@href="fn.bar.html"]' 'bar'
71: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//section[@id="main"]/table//tr[@class="module-item"]/td[@class="docblock-short"]//a[@href="../../../../../subone/fn.foo.html"]' 'other foo'
72: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//section[@id="main"]/table//tr[@class="module-item"]/td[@class="docblock-short"]//a[@href="../../../../../subtwo/fn.bar.html"]' 'other bar'
Encountered 4 errors

------------------------------------------



---- [rustdoc] rustdoc/reexport-check.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/reexport-check" "/checkout/src/test/rustdoc/reexport-check.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/index.html' '//tr[@class="deprecated module-item"]' 'i32'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/index.html' '//tr[@class="module-item"]' 'String'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/index.html' '//td[@class="docblock-short"]' 'Docs in original'
Encountered 3 errors

------------------------------------------



---- [rustdoc] rustdoc/short-docblock-codeblock.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/short-docblock-codeblock" "/checkout/src/test/rustdoc/short-docblock-codeblock.rs"
------------------------------------------
------------------------------------------
Failed to get path ".//*[@class="module-item"]//following-sibling::item-right[@class="docblock-short"]"
------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 263, in iterfind
    selector = _cache[cache_key]
KeyError: ('.//*[@class="module-item"]//following-sibling::item-right[@class="docblock-short"]', None)

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 80, in xpath_tokenizer
    raise KeyError
KeyError

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 439, in check_command
    ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
  File "/checkout/src/etc/htmldocck.py", line 370, in check_tree_text
    for e in tree.findall(path):
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 304, in findall
    return list(iterfind(elem, path, namespaces))
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 277, in iterfind
    selector.append(ops[token[0]](next, token))
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 118, in prepare_descendant
    token = next()
  File "/usr/lib/python3.6/xml/etree/ElementPath.py", line 83, in xpath_tokenizer
    raise SyntaxError("prefix %r not found in prefix map" % prefix)
SyntaxError: prefix 'following-sibling' not found in prefix map
------------------------------------------



---
test result: FAILED. 437 passed; 5 failed; 3 ignored; 0 measured; 0 filtered out; finished in 35.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:21
