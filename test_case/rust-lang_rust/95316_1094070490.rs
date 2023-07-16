plain
failures:

---- [rustdoc] src/test/rustdoc/cross-crate-hidden-assoc-trait-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-hidden-assoc-trait-items" "/checkout/src/test/rustdoc/cross-crate-hidden-assoc-trait-items.rs"
stdout: none
--- stderr -------------------------------
19: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="associatedtype.VisibleAssoc-1"]' 'type VisibleAssoc = ()'
20: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="associatedconstant.VISIBLE_ASSOC-1"]' 'const VISIBLE_ASSOC: ()'
Encountered 2 errors
------------------------------------------


