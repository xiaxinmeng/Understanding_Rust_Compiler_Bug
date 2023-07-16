plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 478 tests
i.................................F..............i...............F.................................. 100/478
............................................................................F....................F.. 200/478
.................i.......F.........................................................i................ 400/478
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.....................F.........................F.........F..F..............F..

---- [rustdoc] rustdoc/const-generics/const-generics-docs.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 157, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/enum.Enum.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/doc-cfg-simplification.rs stdout ----
---- [rustdoc] rustdoc/doc-cfg-simplification.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification" "/checkout/src/test/rustdoc/doc-cfg-simplification.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 171, in goahead
    k = self.parse_starttag(i)
  File "/usr/lib/python3.6/html/parser.py", line 345, in parse_starttag
    self.handle_starttag(tag, attrs)
  File "/checkout/src/etc/htmldocck.py", line 152, in handle_starttag
    self.__builder.start(tag, attrs)
xml.etree.ElementTree.ParseError: multiple elements on top level

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 446, in check_command
    found = get_tree_count(cache.get_tree(c.args[0]), c.args[1])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'globuliferous/ratel/enum.Cosmotellurian.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/intra-doc/issue-82209.rs stdout ----
---- [rustdoc] rustdoc/intra-doc/issue-82209.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/issue-82209" "/checkout/src/test/rustdoc/intra-doc/issue-82209.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 157, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/enum.Foo.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/intra-link-self-cache.rs stdout ----
---- [rustdoc] rustdoc/intra-link-self-cache.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-self-cache" "/checkout/src/test/rustdoc/intra-link-self-cache.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 157, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/enum.E1.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/issue-88600.rs stdout ----
---- [rustdoc] rustdoc/issue-88600.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-88600" "/checkout/src/test/rustdoc/issue-88600.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 171, in goahead
    k = self.parse_starttag(i)
  File "/usr/lib/python3.6/html/parser.py", line 345, in parse_starttag
    self.handle_starttag(tag, attrs)
  File "/checkout/src/etc/htmldocck.py", line 152, in handle_starttag
    self.__builder.start(tag, attrs)
xml.etree.ElementTree.ParseError: multiple elements on top level

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'issue_88600/enum.FooEnum.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/structfields.rs stdout ----
---- [rustdoc] rustdoc/structfields.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/structfields" "/checkout/src/test/rustdoc/structfields.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 157, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'structfields/enum.Qux.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/toggle-item-contents.rs stdout ----
---- [rustdoc] rustdoc/toggle-item-contents.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/toggle-item-contents" "/checkout/src/test/rustdoc/toggle-item-contents.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 157, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 446, in check_command
    found = get_tree_count(cache.get_tree(c.args[0]), c.args[1])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'toggle_item_contents/enum.LargeEnum.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/tuple-struct-fields-doc.rs stdout ----
---- [rustdoc] rustdoc/tuple-struct-fields-doc.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/tuple-struct-fields-doc" "/checkout/src/test/rustdoc/tuple-struct-fields-doc.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 171, in goahead
    k = self.parse_starttag(i)
  File "/usr/lib/python3.6/html/parser.py", line 345, in parse_starttag
    self.handle_starttag(tag, attrs)
  File "/checkout/src/etc/htmldocck.py", line 152, in handle_starttag
    self.__builder.start(tag, attrs)
xml.etree.ElementTree.ParseError: multiple elements on top level

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/enum.Bar.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/type-layout.rs stdout ----
---- [rustdoc] rustdoc/type-layout.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/type-layout" "/checkout/src/test/rustdoc/type-layout.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 157, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'type_layout/enum.WithNiche.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/where.rs stdout ----
---- [rustdoc] rustdoc/where.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where" "/checkout/src/test/rustdoc/where.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 326, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 157, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 490, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 482, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 430, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 328, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/enum.Foxtrot.html': pop from empty stack
------------------------------------------



---
test result: FAILED. 464 passed; 10 failed; 4 ignored; 0 measured; 0 filtered out; finished in 36.89s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:24
