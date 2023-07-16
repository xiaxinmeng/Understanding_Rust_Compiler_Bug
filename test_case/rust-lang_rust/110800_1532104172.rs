plain
.

failures:

---- [rustdoc] tests/rustdoc/custom_code_classes.rs stdout ----
Build completed unsuccessfully in 0:16:05
error: htmldocck failed!
status: exit status: 1
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/custom_code_classes" "/checkout/tests/rustdoc/custom_code_classes.rs"
stdout: none
11: @has check failed
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main-content"]//pre[@class="language-whatever4 huhu-c"]' 'main;'
Encountered 1 errors
------------------------------------------


