plain
---- [rustdoc] tests/rustdoc/issue-25001.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001" "/checkout/tests/rustdoc/issue-25001.rs"
stdout: none
--- stderr -------------------------------
24: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="associatedtype.Item"]//h4[@class="code-header"]' 'type Item = T'
31: @has check failed
 `XPATH PATTERN` did not match
     // @has - '//*[@id="associatedtype.Item-1"]//h4[@class="code-header"]' "type Item = &'a T"
Encountered 2 errors
------------------------------------------


