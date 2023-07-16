
---- [rustdoc] rustdoc/test.rs stdout ----

error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python2.7" "/home/mitaa/dev/rust/src/etc/htmldocck.py" "x86_64-unknown-linux-gnu/test/rustdoc/test.stage1-x86_64-unknown-linux-gnu" "/home/mitaa/dev/rust/src/test/rustdoc/test.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/home/mitaa/dev/rust/src/etc/htmldocck.py", line 397, in <module>
    check(sys.argv[1], get_commands(sys.argv[2]))
  File "/home/mitaa/dev/rust/src/etc/htmldocck.py", line 390, in check
    c.cmd, c.lineno))
RuntimeError: @has check failed at line 1

------------------------------------------
