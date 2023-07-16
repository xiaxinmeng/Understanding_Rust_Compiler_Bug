
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 427 tests
................F................................................................................... 100/427
.................................................................................................... 200/427
.................................................................................................... 300/427
...................................................i................................................ 400/427
...........................
failures:

---- [rustdoc] rustdoc/check-styled-link.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python" "/home/imperio/rust/rust/src/etc/htmldocck.py" "/home/imperio/rust/rust/build/x86_64-unknown-linux-gnu/test/rustdoc/check-styled-link" "/home/imperio/rust/rust/src/test/rustdoc/check-styled-link.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
5: @has check failed
	`XPATH PATTERN` did not match
	// @has foo/struct.Bar.html '//a[@href="../foo/struct.Foo.html"]' 'Foosdbf'

Encountered 1 errors

------------------------------------------



failures:
    [rustdoc] rustdoc/check-styled-link.rs

test result: FAILED. 425 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 44.34s
