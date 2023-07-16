plain
failures:

---- [rustdoc] src/test/rustdoc/static-root-path.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/static-root-path" "/checkout/src/test/rustdoc/static-root-path.rs"
stdout: none
--- stderr -------------------------------
4: @matchesraw check failed
 `PATTERN` did not match
 // @matchesraw - '"/cache/static.files/main-'
11: @matchesraw check failed
 `PATTERN` did not match
 // @matchesraw - '"/cache/static.files/source-script-'
17: @matchesraw check failed
 `PATTERN` did not match
 // @matchesraw - '/cache/static.files/settings-'
Encountered 3 errors
------------------------------------------


