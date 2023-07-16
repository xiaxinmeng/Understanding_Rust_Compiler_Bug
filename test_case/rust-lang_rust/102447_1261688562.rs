plain
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] src/test/rustdoc/toggle-trait-fn.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/toggle-trait-fn" "/checkout/src/test/rustdoc/toggle-trait-fn.rs"
stdout: none
--- stderr -------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has -  '//details[@class="rustdoc-toggle"]//summary//h4[@class="code-header"]' 'is_documented()'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has -  '//details[@class="rustdoc-toggle"]//*[@class="docblock"]' 'is_documented is documented'
10: @has check failed
 `XPATH PATTERN` did not match
 // @has -  '//details[@class="rustdoc-toggle"]//summary//h4[@class="code-header"]' 'is_documented_optional()'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has -  '//details[@class="rustdoc-toggle"]//*[@class="docblock"]' 'is_documented_optional is documented'
Encountered 4 errors
------------------------------------------


