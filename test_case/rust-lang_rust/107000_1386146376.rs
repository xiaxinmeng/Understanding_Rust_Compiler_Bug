plain
---- [rustdoc] checkout/tests/rustdoc/redirect.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/redirect" "/checkout/tests/rustdoc/redirect.rs"
stdout: none
--- stderr -------------------------------
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a[@href="../reexp_stripped/struct.Bar.html"]' 'Bar'
Encountered 1 errors
------------------------------------------


