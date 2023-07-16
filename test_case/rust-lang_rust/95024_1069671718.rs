plain
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/keyword.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/keyword" "/checkout/src/test/rustdoc/keyword.rs"
stdout: none
--- stderr -------------------------------
9: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/keyword.match.html '//a[@class="keyword"]' 'match'
Encountered 1 errors
------------------------------------------


