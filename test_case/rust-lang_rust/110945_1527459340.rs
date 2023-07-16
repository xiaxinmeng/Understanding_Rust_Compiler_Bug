plain
---- [rustdoc] tests/rustdoc/anchors.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/anchors" "/checkout/tests/rustdoc/anchors.rs"
--- expected ---


<section id="associatedtype.Y" class="associatedtype"><h4 class="code-header">type <a href="#associatedtype.Y" class="associatedtype">Y</a> = <a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.u32.html">u32</a></h4></section>

--- actual ---


<section id="associatedtype.Y" class="associatedtype"><h4 class="code-header">pub type <a href="#associatedtype.Y" class="associatedtype">Y</a> = <a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.u32.html">u32</a></h4></section>
--- stderr -------------------------------
--- stderr -------------------------------
text: 'pub type ' != 'type '
children 1 do not match: h4
children 1 do not match: section
43: @snapshot check failed
 Actual snapshot value is different than expected
     // @snapshot no_type_anchor2 - '//*[@id="associatedtype.Y"]'
Encountered 1 errors
------------------------------------------


