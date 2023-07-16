plain
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......................
failures:

---- [rustdoc] src/test/rustdoc/intra-doc/assoc-reexport-super.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/assoc-reexport-super" "/checkout/src/test/rustdoc/intra-doc/assoc-reexport-super.rs"
stdout: none
--- stderr -------------------------------
17: @has check failed
 `PATTERN` did not match
         // @has 'foo/struct.MyNewType.html' '//a[@href="struct.MyNewType.html#associatedconst.Foo"]'
Encountered 1 errors
------------------------------------------


