plain
.................................................................................................... 9400/11762
.................................................................................................... 9500/11762
.............................................................................................i...... 9600/11762
i................................................................................................... 9700/11762
.......................................iiiiiii..iiiiii.i............................................ 9800/11762
.................................................................................................... 10000/11762
.................................................................................................... 10100/11762
.................................................................................................... 10200/11762
.................................................................................................... 10300/11762
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.194 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.48s

 finished in 2.554 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 434 tests
i.......................F................i....F.FF....F............F.....F.......................... 100/434
..................................................................................................F. 200/434
.FF.......F................F.......................FF........F..............F.FF..FF...F...F......F. 300/434
................................................i......................................FF..FFFFF..F. 400/434
.FF...............................
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/const-generics/add-impl.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/add-impl" "/checkout/src/test/rustdoc/const-generics/add-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Simd.html '//div[@id="trait-implementations-list"]/h3/code' 'impl Add<Simd<u8, 16_usize>> for Simd<u8, 16>'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-mut-methods.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-mut-methods" "/checkout/src/test/rustdoc/deref-mut-methods.rs"
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
RuntimeError: Cannot parse an HTML file 'foo/struct.Bar.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/deref-recursive-pathbuf.rs stdout ----
---- [rustdoc] rustdoc/deref-recursive-pathbuf.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf" "/checkout/src/test/rustdoc/deref-recursive-pathbuf.rs"
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
RuntimeError: Cannot parse an HTML file 'foo/struct.Foo.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/deref-recursive.rs stdout ----
---- [rustdoc] rustdoc/deref-recursive.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive" "/checkout/src/test/rustdoc/deref-recursive.rs"
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
RuntimeError: Cannot parse an HTML file 'foo/struct.Foo.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/deref-typedef.rs stdout ----
---- [rustdoc] rustdoc/deref-typedef.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef" "/checkout/src/test/rustdoc/deref-typedef.rs"
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
RuntimeError: Cannot parse an HTML file 'foo/struct.Bar.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/duplicate_impls/issue-33054.rs stdout ----
---- [rustdoc] rustdoc/duplicate_impls/issue-33054.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/issue-33054" "/checkout/src/test/rustdoc/duplicate_impls/issue-33054.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @count check failed
 Expected 1 occurrences but found 0
 // @count - '//*[@id="trait-implementations-list"]/*[@class="impl"]' 1
5: @count check failed
 Expected 1 occurrences but found 0
 // @count - '//*[@id="main"]/*[@class="impl"]' 1
Encountered 2 errors

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
RuntimeError: Cannot parse an HTML file 'foo/struct.TitleList.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/issue-19190.rs stdout ----
---- [rustdoc] rustdoc/issue-19190.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190" "/checkout/src/test/rustdoc/issue-19190.rs"
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
RuntimeError: Cannot parse an HTML file 'issue_19190/struct.Bar.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/issue-19190-2.rs stdout ----
---- [rustdoc] rustdoc/issue-19190-2.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-2" "/checkout/src/test/rustdoc/issue-19190-2.rs"
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
RuntimeError: Cannot parse an HTML file 'issue_19190_2/struct.Bar.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----
---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3" "/checkout/src/test/rustdoc/issue-19190-3.rs"
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
RuntimeError: Cannot parse an HTML file 'issue_19190_3/struct.Foo.html': multiple elements on top level
------------------------------------------


---- [rustdoc] rustdoc/issue-21474.rs stdout ----
---- [rustdoc] rustdoc/issue-21474.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21474" "/checkout/src/test/rustdoc/issue-21474.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
9: @count check failed
 Expected 1 occurrences but found 0
 // @count issue_21474/struct.What.html '//*[@id="trait-implementations-list"]/*[@class="impl"]' 1
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
 // @has - "//div[@id='implementors-list']/h3[@id='impl-MyTrait']//code" "impl<T> MyTrait for T where T: Debug"
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/issue-35169-2.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169-2" "/checkout/src/test/rustdoc/issue-35169-2.rs"
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
RuntimeError: Cannot parse an HTML file 'issue_35169_2/struct.Bar.html': pop from empty stack
------------------------------------------


---- [rustdoc] rustdoc/issue-35169.rs stdout ----
---- [rustdoc] rustdoc/issue-35169.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169" "/checkout/src/test/rustdoc/issue-35169.rs"
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
---
test result: FAILED. 398 passed; 33 failed; 3 ignored; 0 measured; 0 filtered out; finished in 35.58s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:16
