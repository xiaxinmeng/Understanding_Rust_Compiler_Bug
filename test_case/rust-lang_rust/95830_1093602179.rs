plain
test [ui] src/test/ui/lint/lint-removed-cmdline.rs ... ok
test [ui] src/test/ui/lint/lint-renamed.rs ... ok
test [ui] src/test/ui/lint/lint-renamed-cmdline.rs ... ok
test [ui] src/test/ui/lint/lint-output-format.rs ... ok
test [ui] src/test/ui/lint/lint-strict-provenance-fuzzy-casts.rs ... ok
test [ui] src/test/ui/lint/lint-strict-provenance-lossy-casts.rs ... ok
test [ui] src/test/ui/lint/lint-shorthand-field.rs ... ok
test [ui] src/test/ui/lint/lint-temporary-cstring-as-param.rs ... ok
test [ui] src/test/ui/lint/lint-temporary-cstring-as-ptr.rs ... ok
test [ui] src/test/ui/lint/lint-type-limits.rs ... ok
---
failures:

---- [rustdoc] src/test/rustdoc/where.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where" "/checkout/src/test/rustdoc/where.rs"
--- expected ---


<div class="docblock item-decl"><pre class="rust struct"><code>pub struct Simd&lt;T, const LANES:&#160;<a class="primitive" href="https://doc.rust-lang.org/nightly/std/primitive.usize.html">usize</a>&gt;(_) <br /><span class="where">where<br />&#160;&#160;&#160;&#160;T: <a class="trait" href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" title="trait core::marker::Sized">Sized</a></span>;</code></pre></div>

--- actual ---


<div class="docblock item-decl"><pre class="rust struct"><code>pub struct Simd&lt;T, const LANES:&#160;<a class="primitive" href="https://doc.rust-lang.org/1.62.0/std/primitive.usize.html">usize</a>&gt;(_) <br /><span class="where">where<br />&#160;&#160;&#160;&#160;T: <a class="trait" href="https://doc.rust-lang.org/1.62.0/core/marker/trait.Sized.html" title="trait core::marker::Sized">Sized</a></span>;</code></pre></div>
--- stderr -------------------------------
--- stderr -------------------------------
24: @snapshot check failed
 Actual snapshot value is different than expected
 // @snapshot SWhere_Simd_item-decl - '//div[@class="docblock item-decl"]'
Encountered 1 errors
------------------------------------------


