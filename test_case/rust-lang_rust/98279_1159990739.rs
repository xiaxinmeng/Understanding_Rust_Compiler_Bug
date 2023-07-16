plain
failures:

---- [rustdoc] src/test/rustdoc/assoc-consts.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
stdout: none
--- stderr -------------------------------
43: @has check failed
 `XPATH PATTERN` did not match
     // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.F"]' "const F: fn(_: &(dyn ToString + 'static))"
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/fn-type.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/fn-type" "/checkout/src/test/rustdoc/fn-type.rs"
stdout: none
--- stderr -------------------------------
11: @has check failed
 `XPATH PATTERN` did not match
 // @has 'foo/struct.Foo.html' '//span[@id="structfield.generic"]' "generic: fn(val: &T) -> T"
Encountered 1 errors
------------------------------------------


