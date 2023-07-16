plain
failures:

---- [rustdoc] rustdoc/strip-block-doc-comments-stars.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/strip-block-doc-comments-stars" "/checkout/src/test/rustdoc/strip-block-doc-comments-stars.rs"
------------------------------------------
--- expected ---


<div class="docblock"><p>a</p>
</div>

--- actual ---


<div class="docblock"><ul>
<li>
<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="ident">a</span></code></pre></div>
</li>
</ul>
</div>

------------------------------------------
stderr:
------------------------------------------
------------------------------------------
7: @snapshot check failed
 Actual snapshot value is different than expected
 // @snapshot docblock - '//*[@class="rustdoc-toggle top-doc"]//*[@class="docblock"]'
Encountered 1 errors

------------------------------------------

