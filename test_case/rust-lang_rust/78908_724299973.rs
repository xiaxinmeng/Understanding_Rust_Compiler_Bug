
---- [rustdoc] rustdoc/external-macro-src.rs stdout ----
------------------------------------------
12: @has check failed
	`XPATH PATTERN` did not match
	// @has - '//a[@href="https://example.com/src/external_macro_src/external-macro-src.rs.html#8"]' '[src]'
13: @has check failed
	`XPATH PATTERN` did not match
	// @has - '//a[@href="https://example.com/src/external_macro_src/external-macro-src.rs.html#9-13"]' '[src]'
14: @has check failed
	`XPATH PATTERN` did not match
	// @has - '//a[@href="https://example.com/src/external_macro_src/external-macro-src.rs.html#10-12"]' '[src]'

Encountered 3 errors

------------------------------------------


---- [rustdoc] rustdoc/issue-26606.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-26606" "/checkout/src/test/rustdoc/issue-26606.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
10: @has check failed
	`XPATH PATTERN` did not match
	// @has - '//a/@href' '../src/issue_26606_macro/issue-26606-macro.rs.html#3'

Encountered 1 errors

------------------------------------------


---- [rustdoc] rustdoc/thread-local-src.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/thread-local-src" "/checkout/src/test/rustdoc/thread-local-src.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
5: @has check failed
	`XPATH PATTERN` did not match
	// @has foo/constant.FOO.html '//a/@href' 'https://doc.rust-lang.org/nightly/src/std/'

Encountered 1 errors

------------------------------------------



failures:
    [rustdoc] rustdoc/external-macro-src.rs
    [rustdoc] rustdoc/issue-26606.rs
    [rustdoc] rustdoc/thread-local-src.rs
