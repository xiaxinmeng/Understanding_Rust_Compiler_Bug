plain
failures:

---- [rustdoc] src/test/rustdoc/issue-32374.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32374" "/checkout/src/test/rustdoc/issue-32374.rs"
stdout: none
--- stderr -------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has issue_32374/struct.T.html '//*[@class="stab deprecated"]' '👎 Deprecated since 1.0.0: text'
14: @matches check failed
 `XPATH PATTERN` did not match
 // @matches issue_32374/struct.T.html '//*[@class="stab unstable"]' '🔬 This is a nightly-only experimental API. \(test\s#32374\)$'
21: @has check failed
 `XPATH PATTERN` did not match
 // @has issue_32374/struct.U.html '//*[@class="stab deprecated"]' '👎 Deprecated since 1.0.0: deprecated'
23: @has check failed
 `XPATH PATTERN` did not match
 // @has issue_32374/struct.U.html '//*[@class="stab unstable"]' '🔬 This is a nightly-only experimental API. (test #32374)'
Encountered 4 errors
------------------------------------------


