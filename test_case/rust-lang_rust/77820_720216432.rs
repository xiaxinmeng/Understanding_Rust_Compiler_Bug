
---- [rustdoc] rustdoc/assoc-types.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python" "/home/joshua/rustc/src/etc/htmldocck.py" "/home/joshua/rustc/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/home/joshua/rustc/src/test/rustdoc/assoc-types.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
18: @has check failed
	`XPATH PATTERN` did not match
	// @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' 'Output'
