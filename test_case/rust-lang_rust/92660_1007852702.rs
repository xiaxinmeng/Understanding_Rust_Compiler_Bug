plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 491 tests
i......................................F......F...F..iF..FFF............F.FF...F......FF..F....F.F.. 100/491
...F....FF.....................F..........FF.....FF..F...F....F.....F...............F..F............ 200/491
..F........................................F.............F...F...F.............F........F...F..F.F.. 300/491
...........F......F..........i......F...F....................F...F.....F......F.F.F.F..........i...F 400/491
FFF......F..FFF...........F....F...........F.......F....FF.......F..............F..FF......

---- [rustdoc] rustdoc/constructor-imports.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/constructor-imports" "/checkout/src/test/rustdoc/constructor-imports.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 486, in check_command
    found = get_tree_count(cache.get_tree(c.args[0]), c.args[1])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/default-theme.rs stdout ----
---- [rustdoc] rustdoc/default-theme.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-theme" "/checkout/src/test/rustdoc/default-theme.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'default_theme/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/deprecated-future.rs stdout ----
---- [rustdoc] rustdoc/deprecated-future.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-future" "/checkout/src/test/rustdoc/deprecated-future.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'deprecated_future/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/deprecated.rs stdout ----
---- [rustdoc] rustdoc/deprecated.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated" "/checkout/src/test/rustdoc/deprecated.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'deprecated/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/description.rs stdout ----
---- [rustdoc] rustdoc/description.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/description" "/checkout/src/test/rustdoc/description.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/description_default.rs stdout ----
---- [rustdoc] rustdoc/description_default.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/description_default" "/checkout/src/test/rustdoc/description_default.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/cross-crate-links.rs stdout ----
---- [rustdoc] rustdoc/cross-crate-links.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-links" "/checkout/src/test/rustdoc/cross-crate-links.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/index.html': pop from empty stack
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
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 486, in check_command
    found = get_tree_count(cache.get_tree(c.args[0]), c.args[1])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'globuliferous/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/doc-cfg-traits.rs stdout ----
---- [rustdoc] rustdoc/doc-cfg-traits.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-traits" "/checkout/src/test/rustdoc/doc-cfg-traits.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 486, in check_command
    found = get_tree_count(cache.get_tree(c.args[0]), c.args[1])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'myrmecophagous/index.html': pop from empty stack
------------------------------------------


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
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'doc_cfg/unix_only/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/duplicate-cfg.rs stdout ----
---- [rustdoc] rustdoc/duplicate-cfg.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate-cfg" "/checkout/src/test/rustdoc/duplicate-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 562, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 546, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 470, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 342, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/index.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/empty-mod-private.rs stdout ----
---- [rustdoc] rustdoc/empty-mod-private.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-mod-private" "/checkout/src/test/rustdoc/empty-mod-private.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 340, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1360, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.8/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.8/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.8/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem)
  File "/checkout/src/etc/htmldocck.py", line 171, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

---
test result: FAILED. 418 passed; 69 failed; 4 ignored; 0 measured; 0 filtered out; finished in 46.27s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:17:38
