plain
failures:

---- [rustdoc] src/test/rustdoc/extern-default-method.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method" "/checkout/src/test/rustdoc/extern-default-method.rs"
stdout: none
--- stderr -------------------------------
Traceback (most recent call last):
  File "/checkout/src/etc/htmldocck.py", line 626, in <module>
    check(sys.argv[1], get_commands(rust_test_path))
  File "/checkout/src/etc/htmldocck.py", line 610, in check
    check_command(c, cache)
  File "/checkout/src/etc/htmldocck.py", line 565, in check_command
    check_snapshot(snapshot_name, subtree, normalize_to_text)
  File "/checkout/src/etc/htmldocck.py", line 422, in check_snapshot
    or (not normalize_to_text and not compare_tree(actual_tree, ET.XML(expected_str), stderr)) \
  File "/usr/lib/python3.8/xml/etree/ElementTree.py", line 1320, in XML
    parser.feed(text)
xml.etree.ElementTree.ParseError: junk after document element: line 1, column 30



failures:
