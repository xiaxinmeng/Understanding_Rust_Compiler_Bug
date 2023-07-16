plain
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.........................................
failures:

---- [rustdoc] src/test/rustdoc/doc-notable_trait_box_is_not_an_iterator.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-notable_trait_box_is_not_an_iterator" "/checkout/src/test/rustdoc/doc-notable_trait_box_is_not_an_iterator.rs"
stdout: none
--- stderr -------------------------------
30: @!has check failed
 File does not exist 'issue_100320/fn.foo.html'
 // @!has issue_100320/fn.foo.html '//*' 'Notable'
35: @!has check failed
 File does not exist 'issue_100320/fn.bar.html'
 // @!has issue_100320/fn.bar.html '//*' 'Notable'
Encountered 2 errors
------------------------------------------


