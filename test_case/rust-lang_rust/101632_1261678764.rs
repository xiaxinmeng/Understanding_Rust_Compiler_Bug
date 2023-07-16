plain
failures:

---- [rustdoc] src/test/rustdoc/doc-cfg-traits.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-traits" "/checkout/src/test/rustdoc/doc-cfg-traits.rs"
stdout: none
--- stderr -------------------------------
76: @count check failed
 Expected 8 occurrences but found 17
 // @count   - '//*[@class="stab portability"]' 8
100: @count check failed
 Expected 9 occurrences but found 18
 // @count   - '//*[@class="stab portability"]' 9
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/generic-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/generic-impl" "/checkout/src/test/rustdoc/generic-impl.rs"
stdout: none
--- stderr -------------------------------
5: @!has check failed
 `XPATH PATTERN` did not match
 // @!has foo/struct.Bar.html '//*[@id="impl-ToString-for-Bar"]' ''
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/internal.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/internal" "/checkout/src/test/rustdoc/internal.rs"
stdout: none
--- stderr -------------------------------
13: @!has check failed
 `XPATH PATTERN` did not match
 // @!has internal/struct.S.html '//*[@class="stab unstable"]' ''
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-78673.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-78673" "/checkout/src/test/rustdoc/issue-78673.rs"
stdout: none
--- stderr -------------------------------
11: @!has check failed
 `XPATH PATTERN` did not match
 // @!has - '//*[@class="impl has-srclink"]' 'AnAmazingTrait for T'
Encountered 1 errors
------------------------------------------


