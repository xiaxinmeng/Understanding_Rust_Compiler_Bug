plain
failures:

---- [rustdoc] src/test/rustdoc/anchors.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/anchors" "/checkout/src/test/rustdoc/anchors.rs"
--- expected ---


<section id="associatedconstant.YOLO" class="associatedconstant trait-impl has-srclink"><span class="rightside"><a class="srclink" href="../src/foo/anchors.rs.html#31">source</a></span><a href="#associatedconstant.YOLO" class="anchor" /><h4 class="code-header">const <a href="trait.Bar.html#associatedconstant.YOLO" class="constant">YOLO</a>: <a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.u32.html">u32</a> = 0u32</h4></section>

--- actual ---


<section id="associatedtype.T" class="associatedtype trait-impl has-srclink"><a href="#associatedtype.T" class="anchor" /><h4 class="code-header">type <a href="trait.Bar.html#associatedtype.T" class="associatedtype">T</a> = <a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.u32.html">u32</a></h4></section>
--- stderr -------------------------------
--- stderr -------------------------------
Attributes do not match: id='associatedtype.T', id='associatedconstant.YOLO'
children 1 do not match: section
28: @snapshot check failed
 Actual snapshot value is different than expected
     // @snapshot type_anchor - '//*[@id="associatedtype.T"]'
Encountered 1 errors
------------------------------------------


