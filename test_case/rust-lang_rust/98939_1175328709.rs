plain

Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [rustdoc] src/test/rustdoc/double-quote-escape.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/double-quote-escape" "/checkout/src/test/rustdoc/double-quote-escape.rs"
stdout: none
--- stderr -------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="sidebar-elems"]//section//a[@href="#impl-Foo%3Cunsafe%20extern%20%22C%22%20fn()%3E"]' 'Foo<unsafe extern "C" fn()>'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/generic-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
stdout: none
--- stderr -------------------------------
10: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.Foo.html '//*[@class="sidebar-elems"]//section//a[@href="#impl-ToString"]' 'ToString'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-78701.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-78701" "/checkout/src/test/rustdoc/issue-78701.rs"
stdout: none
--- stderr -------------------------------
8: @count check failed
 Expected 1 occurrences but found 0
 // @count - '//*[@class="sidebar"]//a[@href="#impl-AnAmazingTrait"]' 1
9: @count check failed
 Expected 1 occurrences but found 0
 // @count - '//*[@class="sidebar"]//a[@href="#impl-AnAmazingTrait-1"]' 1
Encountered 2 errors
------------------------------------------


