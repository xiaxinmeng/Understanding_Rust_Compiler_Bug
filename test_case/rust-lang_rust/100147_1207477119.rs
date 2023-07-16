plain
i..............................................................i........................ 88/546
........................................................................................ 176/546
........................................................................................ 264/546
........................................................................................ 352/546
................i..................F.................................................... 440/546
....i.F................................................................................. 528/546
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] src/test/rustdoc/macro-indirect-use.rs stdout ----
---- [rustdoc] src/test/rustdoc/macro-indirect-use.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/macro-indirect-use" "/checkout/src/test/rustdoc/macro-indirect-use.rs"
stdout: none
--- stderr -------------------------------
15: @has check failed
 File does not exist 'macro_indirect_use/inner/macro.some_macro.html'
 // @has macro_indirect_use/inner/macro.some_macro.html
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/redirect-rename.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect-rename" "/checkout/src/test/rustdoc/redirect-rename.rs"
stdout: none
--- stderr -------------------------------
15: @has check failed
 File does not exist 'foo/hidden/bar/struct.Thing.html'
         // @has foo/hidden/bar/struct.Thing.html
16: @has check failed
 File does not exist 'foo/hidden/bar/struct.Thing.html'
         // @has - '//p/a' '../../foo/baz/struct.Thing.html'
33: @has check failed
 File does not exist 'foo/baz/struct.Thing.html'
 // @has foo/baz/struct.Thing.html
Encountered 3 errors
------------------------------------------


