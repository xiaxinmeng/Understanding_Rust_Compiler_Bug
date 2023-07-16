plain
---- [rustdoc] tests/rustdoc/doc-cfg-simplification.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification" "/checkout/tests/rustdoc/doc-cfg-simplification.rs"
stdout: none
--- stderr -------------------------------
70: @count check failed
 Expected 2 occurrences but found 3
 Expected 2 occurrences but found 3
     // @count   - '//*[@class="stab portability"]' 2
72: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature rutherford'
Encountered 2 errors
------------------------------------------



---- [rustdoc] tests/rustdoc/tuple-struct-fields-doc.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/tuple-struct-fields-doc" "/checkout/tests/rustdoc/tuple-struct-fields-doc.rs"
stdout: none
7: @has check failed
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main-content"]/div[@class="docblock"]' 'hello'
11: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main-content"]/div[@class="docblock"]' 'not hello'
Encountered 2 errors
------------------------------------------


