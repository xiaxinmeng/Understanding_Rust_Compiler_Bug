
---- [rustdoc] rustdoc/test.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/home/mitaa/dev/rust/src/etc/htmldocck.py" "x86_64-unknown-linux-gnu/test/rustdoc/test.stage1-x86_64-unknown-linux-gnu" "/home/mitaa/dev/rust/src/test/rustdoc/test.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
1: Tried to use the previous path in the first command
    // @has -
4: @has check failed
    File does not exist 'test/doesnt_exist.html'
    // @has test/doesnt_exist.html
5: @has check failed
    `XPATH PATTERN` did not match
    // @has test/index.html '//*[@class="rust trait"]' 'trait Deref2 {'
6: Unimplemented @valid-links
    // @valid-links

Encountered 4 errors

------------------------------------------
