plain
failures:

---- [rustdoc] src/test/rustdoc/macro_rules-matchers.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/macro_rules-matchers" "/checkout/src/test/rustdoc/macro_rules-matchers.rs"
stdout: none
--- stderr -------------------------------
10: @count check failed
 Expected 1 occurrences but found 0
 // @count - '//pre[@class="rust macro"]//span[@class="op"]' 1
18: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//span[@class="op"]' '+'
Encountered 2 errors
------------------------------------------


