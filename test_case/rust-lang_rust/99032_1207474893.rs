plain
failures:

---- [rustdoc] src/test/rustdoc/cross-crate-hidden-assoc-trait-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-hidden-assoc-trait-items" "/checkout/src/test/rustdoc/cross-crate-hidden-assoc-trait-items.rs"
stdout: none
--- stderr -------------------------------
16: @count check failed
 Expected 2 occurrences but found 3
 // @count - '//*[@class="impl-items"]/section' 2
Encountered 1 errors
------------------------------------------


