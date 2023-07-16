plain

Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [rustdoc] src/test/rustdoc/codeblock-title.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/codeblock-title" "/checkout/src/test/rustdoc/codeblock-title.rs"
stdout: none
--- stderr -------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/fn.bar.html '//*[@class="tooltip compile_fail"]' "ⓘ"
4: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/fn.bar.html '//*[@class="tooltip ignore"]' "ⓘ"
5: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/fn.bar.html '//*[@class="tooltip should_panic"]' "ⓘ"
Encountered 3 errors
------------------------------------------


