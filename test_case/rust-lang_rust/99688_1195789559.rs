plain
failures:

---- [rustdoc] src/test/rustdoc/const-value.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-value" "/checkout/src/test/rustdoc/const-value.rs"
--- expected ---


<code>pub const REC: <a class="struct" href="struct.Record.html" title="struct consts::Record">Record</a>&lt;'_&gt; = <a class="struct" href="struct.Record.html" title="struct consts::Record">Record</a> { <a class="structfield" href="struct.Record.html#structfield.one" title="field one">one</a>: "thriving", <a class="structfield" href="struct.Record.html#structfield.two" title="field two">two</a>: (180,) };</code>

--- actual ---


<code>pub const REC: <a class="struct" href="struct.Record.html" title="struct consts::Record">Record</a>&lt;'static&gt; = <a class="struct" href="struct.Record.html" title="struct consts::Record">Record</a> { <a class="structfield" href="struct.Record.html#structfield.one" title="field one">one</a>: "thriving", <a class="structfield" href="struct.Record.html#structfield.two" title="field two">two</a>: (180,) };</code>
--- stderr -------------------------------
26: @has check failed
26: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock item-decl"]//code' 'pub const CONCATENATED: &str = "[0, +∞)";'
41: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="docblock item-decl"]//code' "const REC: Record<'_> = Record { one: \"thriving\", two: (180,) }"
tail: "<'static> = " != "<'_> = "
children 1 do not match: a
children 1 do not match: code
43: @snapshot check failed
 Actual snapshot value is different than expected
 // @snapshot rec - '//*[@class="docblock item-decl"]//code'
Encountered 3 errors
------------------------------------------


