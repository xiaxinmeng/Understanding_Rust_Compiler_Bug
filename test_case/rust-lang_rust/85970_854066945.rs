plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 444 tests
i.......F...................F............i.F................F.................F..................... 100/444
......F.....F........F......................................................F...................F... 200/444
..........................F.F.............F......................................................... 300/444
................F.................................i................................................. 400/444
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.................F.F....................F...

---- [rustdoc] rustdoc/assoc-consts.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'assoc_consts/trait.Foo.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/const-generics/const-generic-slice.rs stdout ----
---- [rustdoc] rustdoc/const-generics/const-generic-slice.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generic-slice" "/checkout/src/test/rustdoc/const-generics/const-generic-slice.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/trait.Array.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/default-trait-method.rs stdout ----
---- [rustdoc] rustdoc/default-trait-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method" "/checkout/src/test/rustdoc/default-trait-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'default_trait_method/trait.Item.html': pop from empty stack
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
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'doc_cfg/unix_only/trait.ArmOnly.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/ensure-src-link.rs stdout ----
---- [rustdoc] rustdoc/ensure-src-link.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ensure-src-link" "/checkout/src/test/rustdoc/ensure-src-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 171, in goahead
    k = self.parse_starttag(i)
  File "/usr/lib/python3.6/html/parser.py", line 345, in parse_starttag
    self.handle_starttag(tag, attrs)
  File "/checkout/src/etc/htmldocck.py", line 150, in handle_starttag
    self.__builder.start(tag, attrs)
xml.etree.ElementTree.ParseError: multiple elements on top level

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/trait.Iterator.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/inline_cross/default-trait-method.rs stdout ----
---- [rustdoc] rustdoc/inline_cross/default-trait-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/default-trait-method" "/checkout/src/test/rustdoc/inline_cross/default-trait-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'default_trait_method/trait.Item.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----
---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/trait.MyTrait.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/inline_cross/issue-32881.rs stdout ----
---- [rustdoc] rustdoc/inline_cross/issue-32881.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-32881" "/checkout/src/test/rustdoc/inline_cross/issue-32881.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 171, in goahead
    k = self.parse_starttag(i)
  File "/usr/lib/python3.6/html/parser.py", line 345, in parse_starttag
    self.handle_starttag(tag, attrs)
  File "/checkout/src/etc/htmldocck.py", line 150, in handle_starttag
    self.__builder.start(tag, attrs)
xml.etree.ElementTree.ParseError: multiple elements on top level

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'issue_32881/trait.Bar.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/intra-doc/self.rs stdout ----
---- [rustdoc] rustdoc/intra-doc/self.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/self" "/checkout/src/test/rustdoc/intra-doc/self.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'foo/trait.MyTrait.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/issue-19055.rs stdout ----
---- [rustdoc] rustdoc/issue-19055.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19055" "/checkout/src/test/rustdoc/issue-19055.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 171, in goahead
    k = self.parse_starttag(i)
  File "/usr/lib/python3.6/html/parser.py", line 345, in parse_starttag
    self.handle_starttag(tag, attrs)
  File "/checkout/src/etc/htmldocck.py", line 150, in handle_starttag
    self.__builder.start(tag, attrs)
xml.etree.ElementTree.ParseError: multiple elements on top level

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'issue_19055/trait.Any.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/issue-28478.rs stdout ----
---- [rustdoc] rustdoc/issue-28478.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28478" "/checkout/src/test/rustdoc/issue-28478.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 486, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/checkout/src/etc/htmldocck.py", line 478, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 426, in check_command
    tree = cache.get_tree(c.args[0])
  File "/checkout/src/etc/htmldocck.py", line 325, in get_tree
    raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
RuntimeError: Cannot parse an HTML file 'issue_28478/trait.Bar.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/issue-29503.rs stdout ----
---- [rustdoc] rustdoc/issue-29503.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 323, in get_tree
    tree = ET.fromstringlist(f.readlines(), CustomHTMLParser())
  File "/usr/lib/python3.6/xml/etree/ElementTree.py", line 1354, in fromstringlist
    parser.feed(text)
  File "/usr/lib/python3.6/html/parser.py", line 111, in feed
    self.goahead(0)
  File "/usr/lib/python3.6/html/parser.py", line 173, in goahead
    k = self.parse_endtag(i)
  File "/usr/lib/python3.6/html/parser.py", line 421, in parse_endtag
    self.handle_endtag(elem.lower())
  File "/checkout/src/etc/htmldocck.py", line 155, in handle_endtag
    self.__builder.end(tag)
IndexError: pop from empty stack

---
test result: FAILED. 424 passed; 17 failed; 3 ignored; 0 measured; 0 filtered out; finished in 30.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:09
