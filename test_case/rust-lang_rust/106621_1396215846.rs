plain
---- [rustdoc] checkout/tests/rustdoc/playground-arg.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg" "/checkout/tests/rustdoc/playground-arg.rs"
stdout: none
--- stderr -------------------------------
13: @matches check failed
 `XPATH PATTERN` did not match
 // @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=%23!%5Ballow(unused)%5D%0Aextern%20crate%20r%23foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D&edition=2015"]' "Run"
Encountered 1 errors
------------------------------------------


