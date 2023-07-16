plain
failures:

---- [rustdoc] src/test/rustdoc/where.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/obj/build/tmp/distcheck/src/etc/htmldocck.py" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/rustdoc/where" "/checkout/obj/build/tmp/distcheck/src/test/rustdoc/where.rs"
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




failures:
    [rustdoc] src/test/rustdoc/where.rs

test result: FAILED. 513 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 29.05s

Build completed unsuccessfully in 0:18:45
Makefile:42: recipe for target 'check' failed
make: *** [check] Error 1
