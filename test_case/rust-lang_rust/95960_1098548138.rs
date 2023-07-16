plain
failures:

---- [rustdoc] src/test/rustdoc/inline_cross/macros.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macros" "/checkout/src/test/rustdoc/inline_cross/macros.rs"
stdout: none
--- stderr -------------------------------
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' '../src/macros/macros.rs.html#9-11'
Encountered 1 errors
------------------------------------------


