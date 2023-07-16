plain
failures:

---- [rustdoc] src/test/rustdoc/doc-notable_trait_box_is_not_an_iterator.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-notable_trait_box_is_not_an_iterator" "/checkout/src/test/rustdoc/doc-notable_trait_box_is_not_an_iterator.rs"
stdout: none
--- stderr -------------------------------
30: @!has check failed
 `XPATH PATTERN` did not match
 // @!has doc_notable_trait_box_is_not_an_iterator/fn.foo.html '//*' 'Notable'
35: @!has check failed
 `XPATH PATTERN` did not match
 // @!has doc_notable_trait_box_is_not_an_iterator/fn.bar.html '//*' 'Notable'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/intra-doc/prim-self.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-self/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-self" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/prim-self.rs"
stdout: none
--- stderr -------------------------------
error: requires `sized` lang_item
error: aborting due to previous error
------------------------------------------


