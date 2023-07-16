plain
failures:

---- [rustdoc] src/test/rustdoc/static-root-path.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/static-root-path" "/checkout/src/test/rustdoc/static-root-path.rs"
stdout: none
--- stderr -------------------------------
13: @matches check failed
 `PATTERN` did not match
 // @matches - '"\.\./\.\./source-files.js"'
14: @!matches check failed
 `PATTERN` did not match
 // @!matches - '"/cache/source-files\.js"'
Encountered 2 errors
------------------------------------------


