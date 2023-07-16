plain
---- [rustdoc] tests/rustdoc/intra-doc/self.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/self" "/checkout/tests/rustdoc/intra-doc/self.rs"
stdout: none
--- stderr -------------------------------
51: @has check failed
 `XPATH PATTERN` did not match
     // @has foo/union.MyUnion.html '//a/@href' 'union.MyUnion.html#structfield.union_field'
Encountered 1 errors
------------------------------------------



---- [rustdoc] tests/rustdoc/type-layout.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/type-layout" "/checkout/tests/rustdoc/type-layout.rs"
stdout: none
--- stderr -------------------------------
18: @hasraw check failed
 `PATTERN` did not match
 // @hasraw type_layout/union.Baz.html 'Size: '
19: @hasraw check failed
 `PATTERN` did not match
 // @hasraw - ' bytes'
Encountered 2 errors
------------------------------------------


