plain
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

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


