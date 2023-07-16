plain
failures:

---- [rustdoc] rustdoc/namespaces.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/namespaces" "/checkout/src/test/rustdoc/namespaces.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
12: @has check failed
 File does not exist 'namespaces/fn.sync.html'
 // @has namespaces/fn.sync.html
14: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//a/@href' 'fn.sync.html'
Encountered 2 errors

------------------------------------------

