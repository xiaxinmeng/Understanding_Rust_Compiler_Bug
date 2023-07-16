plain
failures:

---- [rustdoc] rustdoc/associated-consts.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/associated-consts" "/checkout/src/test/rustdoc/associated-consts.rs"
stdout: none
--- stderr -------------------------------
36: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="sidebar-title"]' 'Associated Constants'
37: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@class="sidebar-elems"]//a' 'FOO'
47: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//h3[@class="sidebar-title"]' 'Associated Constants'
48: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//div[@class="sidebar-elems"]//a' 'FOO'
Encountered 4 errors
------------------------------------------


