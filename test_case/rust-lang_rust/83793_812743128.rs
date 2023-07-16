plain
.................................................................................................... 9400/11733
.................................................................................................... 9500/11733
..........................................................................i......i.................. 9600/11733
.................................................................................................... 9700/11733
....................iiiiiii..iiiiii.i............................................................... 9800/11733
.................................................................................................... 10000/11733
.................................................................................................... 10100/11733
.................................................................................................... 10200/11733
.................................................................................................... 10300/11733
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.105 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.366 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 22.30s
     Running build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-b4a569680b329f7f

running 53 tests
..............................F..F...................

---- html::highlight::tests::test_html_highlighting stdout ----



error: expect test failed
   --> fixtures/sample.html

You can update all `expect![[]]` tests by running:

    env UPDATE_EXPECT=1 cargo test

To update a single test, place the cursor on `expect` token and use `run` feature of rust-analyzer.
Expect:
----

<style>
<style>
.kw { color: #8959A8; }
.kw-2, .prelude-ty { color: #4271AE; }
.number, .string { color: #718C00; }
.self, .bool-val, .prelude-val, .attribute, .attribute .ident { color: #C82829; }
.macro, .macro-nonterminal { color: #3E999F; }
.lifetime { color: #B76514; }
.question-mark { color: #ff9011; }
</style>
<pre><code><span class="attribute">#![<span class="ident">crate_type</span> <span class="op">=</span> <span class="string">&quot;lib&quot;</span>]</span>

<span class="attribute">#[<span class="ident">cfg</span>(<span class="ident">target_os</span> <span class="op">=</span> <span class="string">&quot;linux&quot;</span>)]</span>
<span class="kw">fn</span> <span class="ident">main</span>() {
    <span class="kw">let</span> <span class="ident">foo</span> <span class="op">=</span> <span class="bool-val">true</span> <span class="op">&amp;&amp;</span> <span class="bool-val">false</span> <span class="op">|</span><span class="op">|</span> <span class="bool-val">true</span>;
    <span class="kw">let</span> <span class="kw">_</span>: <span class="kw-2">*</span><span class="kw">const</span> () <span class="op">=</span> <span class="number">0</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="op">&amp;&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">*</span><span class="ident">foo</span>;
    <span class="macro">mac</span><span class="macro">!</span>(<span class="ident">foo</span>, <span class="kw-2">&amp;</span><span class="kw-2">mut</span> <span class="ident">bar</span>);
    <span class="macro">assert</span><span class="macro">!</span>(<span class="self">self</span>.<span class="ident">length</span> <span class="op">&lt;</span> <span class="ident">N</span> <span class="op">&amp;&amp;</span> <span class="ident">index</span> <span class="op">&lt;</span><span class="op">=</span> <span class="self">self</span>.<span class="ident">length</span>);


<span class="macro">macro_rules</span><span class="macro">!</span> <span class="ident">bar</span> {
    (<span class="macro-nonterminal">$</span><span class="macro-nonterminal">foo</span>:<span class="ident">tt</span>) <span class="op">=</span><span class="op">&gt;</span> {};
}
</code></pre>
----

Actual:
----
----

<style>
.kw { color: #8959A8; }
.kw-2, .prelude-ty { color: #4271AE; }
.number, .string { color: #718C00; }
.self, .bool-val, .prelude-val, .attribute, .attribute .ident { color: #C82829; }
.macro, .macro-nonterminal { color: #3E999F; }
.lifetime { color: #B76514; }
.question-mark { color: #ff9011; }
</style>
<pre><code><span class="attribute">#![<span class="ident">crate_type</span> <span class="op">=</span> <span class="string">&quot;lib&quot;</span>]</span>

<span class="attribute">#[<span class="ident">cfg</span>(<span class="ident">target_os</span> <span class="op">=</span> <span class="string">&quot;linux&quot;</span>)]</span>
<span class="kw">fn</span> <span class="ident">main</span>() {
error: test failed, to rerun pass '-p rustdoc --lib'
    <span class="kw">let</span> <span class="ident">foo</span> <span class="op">=</span> <span class="bool-val">true</span> <span class="op">&amp;&amp;</span> <span class="bool-val">false</span> <span class="op">|</span><span class="op">|</span> <span class="bool-val">true</span>;
    <span class="kw">let</span> <span class="kw">_</span>: <span class="kw-2">*</span><span class="kw">const</span> () <span class="op">=</span> <span class="number">0</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="op">&amp;&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">*</span><span class="ident">foo</span>;
    <span class="macro">mac!</span>(<span class="ident">foo</span>, <span class="kw-2">&amp;</span><span class="kw-2">mut</span> <span class="ident">bar</span>);
    <span class="macro">assert!</span>(<span class="self">self</span>.<span class="ident">length</span> <span class="op">&lt;</span> <span class="ident">N</span> <span class="op">&amp;&amp;</span> <span class="ident">index</span> <span class="op">&lt;</span><span class="op">=</span> <span class="self">self</span>.<span class="ident">length</span>);


<span class="macro">macro_rules!</span> <span class="ident">bar</span> {
    (<span class="macro-nonterminal">$</span><span class="macro-nonterminal">foo</span>:<span class="ident">tt</span>) <span class="op">=</span><span class="op">&gt;</span> {};
}
</code></pre>
----

Diff:
----
----

<style>
.kw { color: #8959A8; }
.kw-2, .prelude-ty { color: #4271AE; }
.number, .string { color: #718C00; }
.self, .bool-val, .prelude-val, .attribute, .attribute .ident { color: #C82829; }
.macro, .macro-nonterminal { color: #3E999F; }
.lifetime { color: #B76514; }
.question-mark { color: #ff9011; }
</style>
<pre><code><span class="attribute">#![<span class="ident">crate_type</span> <span class="op">=</span> <span class="string">&quot;lib&quot;</span>]</span>

<span class="attribute">#[<span class="ident">cfg</span>(<span class="ident">target_os</span> <span class="op">=</span> <span class="string">&quot;linux&quot;</span>)]</span>
<span class="kw">fn</span> <span class="ident">main</span>() {
    <span class="kw">let</span> <span class="ident">foo</span> <span class="op">=</span> <span class="bool-val">true</span> <span class="op">&amp;&amp;</span> <span class="bool-val">false</span> <span class="op">|</span><span class="op">|</span> <span class="bool-val">true</span>;
    <span class="kw">let</span> <span class="kw">_</span>: <span class="kw-2">*</span><span class="kw">const</span> () <span class="op">=</span> <span class="number">0</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="op">&amp;&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">*</span><span class="ident">foo</span>;
    <span class="macro">mac!</span>(<span class="ident">foo</span>, <span class="kw-2">&amp;</span><span class="kw-2">mut</span> <span class="ident">bar</span>);
    <span class="macro">assert!</span>(<span class="self">self</span>.<span class="ident">length</span> <span class="op">&lt;</span> <span class="ident">N</span> <span class="op">&amp;&amp;</span> <span class="ident">index</span> <span class="op">&lt;</span><span class="op">=</span> <span class="self">self</span>.<span class="ident">length</span>);
    <span class="macro">mac</span><span class="macro">!</span>(<span class="ident">foo</span>, <span class="kw-2">&amp;</span><span class="kw-2">mut</span> <span class="ident">bar</span>);
    <span class="macro">assert</span><span class="macro">!</span>(<span class="self">self</span>.<span class="ident">length</span> <span class="op">&lt;</span> <span class="ident">N</span> <span class="op">&amp;&amp;</span> <span class="ident">index</span> <span class="op">&lt;</span><span class="op">=</span> <span class="self">self</span>.<span class="ident">length</span>);


<span class="macro">macro_rules!</span> <span class="ident">bar</span> {
<span class="macro">macro_rules</span><span class="macro">!</span> <span class="ident">bar</span> {
    (<span class="macro-nonterminal">$</span><span class="macro-nonterminal">foo</span>:<span class="ident">tt</span>) <span class="op">=</span><span class="op">&gt;</span> {};
}
</code></pre>

----



---- html::highlight::tests::test_dos_backline stdout ----


error: expect test failed
   --> fixtures/dos_line.html
Expect:
----
----
<span class="kw">pub</span> <span class="kw">fn</span> <span class="ident">foo</span>() {
<span class="macro">println</span><span class="macro">!</span>(<span class="string">&quot;foo&quot;</span>);

----

Actual:
Actual:
----
<span class="kw">pub</span> <span class="kw">fn</span> <span class="ident">foo</span>() {
<span class="macro">println!</span>(<span class="string">&quot;foo&quot;</span>);

----

Diff:
Diff:
----
<span class="kw">pub</span> <span class="kw">fn</span> <span class="ident">foo</span>() {
<span class="macro">println!</span>(<span class="string">&quot;foo&quot;</span>);
<span class="macro">println</span><span class="macro">!</span>(<span class="string">&quot;foo&quot;</span>);


----

---
test result: FAILED. 51 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:16
