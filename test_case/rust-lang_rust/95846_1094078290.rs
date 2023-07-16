plain
failures:

---- [rustdoc] src/test/rustdoc/where.rs stdout ----

error: htmldocck failed!
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/i686-unknown-linux-gnu/test/rustdoc/where" "/checkout/src/test/rustdoc/where.rs"
--- expected ---


<div class="docblock item-decl"><pre class="rust trait"><code>pub trait TraitWhere {
    type <a href="#associatedtype.Item" class="associatedtype">Item</a>&lt;'a&gt;<br />&#160;&#160;&#160; <span class="where">where<br />&#160;&#160;&#160;&#160;&#160;&#160;&#160;&#160;Self: 'a</span>;
}</code></pre></div>

--- actual ---


<div class="docblock item-decl"><pre class="rust trait"><code>pub trait TraitWhere {
    type <a class="associatedtype" href="#associatedtype.Item">Item</a>&lt;'a&gt;<br />&#160;&#160;&#160; <span class="where">where<br />&#160;&#160;&#160;&#160;&#160;&#160;&#160;&#160;Self: 'a</span>;
}</code></pre></div>
--- stderr -------------------------------
--- stderr -------------------------------
30: @snapshot check failed
 Actual snapshot value is different than expected
 // @snapshot SWhere_TraitWhere_item-decl - '//div[@class="docblock item-decl"]'
Encountered 1 errors
------------------------------------------


