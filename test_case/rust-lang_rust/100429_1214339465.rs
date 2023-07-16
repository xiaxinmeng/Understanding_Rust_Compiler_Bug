plain
failures:

---- [rustdoc] src/test/rustdoc/macro_rules-matchers.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/macro_rules-matchers" "/checkout/src/test/rustdoc/macro_rules-matchers.rs"
stdout: none
--- stderr -------------------------------
8: Invalid number of @has arguments
 // @has - ' todo {'
Encountered 1 errors
------------------------------------------


