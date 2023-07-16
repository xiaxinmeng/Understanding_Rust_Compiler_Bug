plain
failures:

---- [rustdoc] src/test/rustdoc/logo-class-default.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/logo-class-default" "/checkout/src/test/rustdoc/logo-class-default.rs"
stdout: none
--- stderr -------------------------------
3: @has check failed
 `XPATH PATTERN` did not match
 // @has logo_class_default/struct.SomeStruct.html '//*[@class="sub-logo-container"]/img[@class="rust-logo"]' ''
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/logo-class.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/logo-class" "/checkout/src/test/rustdoc/logo-class.rs"
stdout: none
--- stderr -------------------------------
8: @has check failed
 `XPATH PATTERN` did not match
 // @has logo_class/struct.SomeStruct.html '//*[@class="sub-logo-container"]/img[@src="https://raw.githubusercontent.com/sagebind/isahc/master/media/isahc.svg.png"]' ''
Encountered 1 errors
------------------------------------------


