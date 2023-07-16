plain
    Finished release [optimized] target(s) in 25.34s
     Running unittests lib.rs (build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-aa8469e226b20717)

running 84 tests
..................................FF......F...F.....................................

---- html::highlight::tests::test_decorations stdout ----



error: expect test failed
   --> fixtures/decorations.html

You can update all `expect![[]]` tests by running:

    env UPDATE_EXPECT=1 cargo test

To update a single test, place the cursor on `expect` token and use `run` feature of rust-analyzer.
Expect:
----
----
<span class="example"><span class="kw">let</span> <span class="ident">x</span> <span class="op">=</span> <span class="number">1</span>;</span>
<span class="kw">let</span> <span class="ident">y</span> <span class="op">=</span> <span class="number">2</span>;

Actual:
----
----
<span class="example"><span class="kw">let</span> <span class="ident">x</span> = <span class="number">1</span>;</span>
<span class="kw">let</span> <span class="ident">y</span> = <span class="number">2</span>;

Diff:
----
----
<span class="example"><span class="kw">let</span> <span class="ident">x</span> = <span class="number">1</span>;</span>
<span class="kw">let</span> <span class="ident">y</span> = <span class="number">2</span>;
<span class="example"><span class="kw">let</span> <span class="ident">x</span> <span class="op">=</span> <span class="number">1</span>;</span>
<span class="kw">let</span> <span class="ident">y</span> <span class="op">=</span> <span class="number">2</span>;
----


---- html::highlight::tests::test_keyword_highlight stdout ----
---- html::highlight::tests::test_keyword_highlight stdout ----


error: expect test failed
   --> fixtures/highlight.html

Expect:
----
<span class="kw">use</span> <span class="ident"><span class="kw">crate</span>::a::foo</span>;
<span class="kw">use</span> <span class="ident"><span class="self">self</span>::whatever</span>;
<span class="kw">let</span> <span class="ident">x</span> <span class="op">=</span> <span class="ident"><span class="kw">super</span>::b::foo</span>;
<span class="kw">let</span> <span class="ident">y</span> <span class="op">=</span> <span class="ident"><span class="self">Self</span>::whatever</span>;

Actual:
----
----
<span class="kw">use</span> <span class="ident"><span class="kw">crate</span>::a::foo</span>;
<span class="kw">use</span> <span class="ident"><span class="self">self</span>::whatever</span>;
<span class="kw">let</span> <span class="ident">x</span> = <span class="ident"><span class="kw">super</span>::b::foo</span>;
<span class="kw">let</span> <span class="ident">y</span> = <span class="ident"><span class="self">Self</span>::whatever</span>;

Diff:
----
----
<span class="kw">use</span> <span class="ident"><span class="kw">crate</span>::a::foo</span>;
<span class="kw">use</span> <span class="ident"><span class="self">self</span>::whatever</span>;
<span class="kw">let</span> <span class="ident">x</span> = <span class="ident"><span class="kw">super</span>::b::foo</span>;
<span class="kw">let</span> <span class="ident">y</span> = <span class="ident"><span class="self">Self</span>::whatever</span>;
<span class="kw">let</span> <span class="ident">x</span> <span class="op">=</span> <span class="ident"><span class="kw">super</span>::b::foo</span>;
<span class="kw">let</span> <span class="ident">y</span> <span class="op">=</span> <span class="ident"><span class="self">Self</span>::whatever</span>;
----


---- html::highlight::tests::test_html_highlighting stdout ----
---

Expect:
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
error: test failed, to rerun pass '-p rustdoc --lib'
<pre><code><span class="attribute">#![<span class="ident">crate_type</span> <span class="op">=</span> <span class="string">&quot;lib&quot;</span>]</span>

<span class="kw">use</span> <span class="ident">std::path</span>::{<span class="ident">Path</span>, <span class="ident">PathBuf</span>};

<span class="attribute">#[<span class="ident">cfg</span>(<span class="ident">target_os</span> <span class="op">=</span> <span class="string">&quot;linux&quot;</span>)]</span>
<span class="kw">fn</span> <span class="ident">main</span>() -&gt; () {
    <span class="kw">let</span> <span class="ident">foo</span> <span class="op">=</span> <span class="bool-val">true</span> <span class="op">&amp;&amp;</span> <span class="bool-val">false</span> <span class="op">|</span><span class="op">|</span> <span class="bool-val">true</span>;
    <span class="kw">let</span> <span class="kw">_</span>: <span class="kw-2">*const</span> () <span class="op">=</span> <span class="number">0</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="op">&amp;&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">*</span><span class="ident">foo</span>;
    <span class="macro">mac!</span>(<span class="ident">foo</span>, <span class="kw-2">&amp;mut</span> <span class="ident">bar</span>);
    <span class="macro">assert!</span>(<span class="self">self</span>.<span class="ident">length</span> <span class="op">&lt;</span> <span class="ident">N</span> <span class="op">&amp;&amp;</span> <span class="ident">index</span> <span class="op">&lt;</span><span class="op">=</span> <span class="self">self</span>.<span class="ident">length</span>);
    <span class="ident">::std::env::var</span>(<span class="string">&quot;gateau&quot;</span>).<span class="ident">is_ok</span>();
    <span class="attribute">#[<span class="ident">rustfmt::skip</span>]</span>
    <span class="kw">let</span> <span class="ident">s</span>:<span class="ident">std::path::PathBuf</span> <span class="op">=</span> <span class="ident">std::path::PathBuf::new</span>();
    <span class="kw">let</span> <span class="kw-2">mut</span> <span class="ident">s</span> <span class="op">=</span> <span class="ident">String::new</span>();

    <span class="kw">match</span> <span class="kw-2">&amp;</span><span class="ident">s</span> {
        <span class="kw-2">ref</span> <span class="kw-2">mut</span> <span class="ident">x</span> =&gt; {}
}


<span class="macro">macro_rules!</span> <span class="ident">bar</span> {
    (<span class="macro-nonterminal">$</span><span class="macro-nonterminal">foo</span>:<span class="ident">tt</span>) =&gt; {};
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
<pre><code><span class="attribute">#![<span class="ident">crate_type</span> = <span class="string">&quot;lib&quot;</span>]</span>

<span class="kw">use</span> <span class="ident">std::path</span>::{<span class="ident">Path</span>, <span class="ident">PathBuf</span>};

<span class="attribute">#[<span class="ident">cfg</span>(<span class="ident">target_os</span> = <span class="string">&quot;linux&quot;</span>)]</span>
<span class="kw">fn</span> <span class="ident">main</span>() -&gt; () {
    <span class="kw">let</span> <span class="ident">foo</span> = <span class="bool-val">true</span> &amp;&amp; <span class="bool-val">false</span> || <span class="bool-val">true</span>;
    <span class="kw">let</span> <span class="kw">_</span>: <span class="kw-2">*const</span> () = <span class="number">0</span>;
    <span class="kw">let</span> <span class="kw">_</span> = <span class="kw-2">&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> = &amp;&amp;<span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> = <span class="kw-2">*</span><span class="ident">foo</span>;
    <span class="macro">mac!</span>(<span class="ident">foo</span>, <span class="kw-2">&amp;mut</span> <span class="ident">bar</span>);
    <span class="macro">assert!</span>(<span class="self">self</span>.<span class="ident">length</span> &lt; <span class="ident">N</span> &amp;&amp; <span class="ident">index</span> &lt;= <span class="self">self</span>.<span class="ident">length</span>);
    <span class="ident">::std::env::var</span>(<span class="string">&quot;gateau&quot;</span>).<span class="ident">is_ok</span>();
    <span class="attribute">#[<span class="ident">rustfmt::skip</span>]</span>
    <span class="kw">let</span> <span class="ident">s</span>:<span class="ident">std::path::PathBuf</span> = <span class="ident">std::path::PathBuf::new</span>();
    <span class="kw">let</span> <span class="kw-2">mut</span> <span class="ident">s</span> = <span class="ident">String::new</span>();

    <span class="kw">match</span> <span class="kw-2">&amp;</span><span class="ident">s</span> {
        <span class="kw-2">ref</span> <span class="kw-2">mut</span> <span class="ident">x</span> =&gt; {}
}


<span class="macro">macro_rules!</span> <span class="ident">bar</span> {
    (<span class="macro-nonterminal">$</span><span class="macro-nonterminal">foo</span>:<span class="ident">tt</span>) =&gt; {};
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
<pre><code><span class="attribute">#![<span class="ident">crate_type</span> = <span class="string">&quot;lib&quot;</span>]</span>
<pre><code><span class="attribute">#![<span class="ident">crate_type</span> <span class="op">=</span> <span class="string">&quot;lib&quot;</span>]</span>

<span class="kw">use</span> <span class="ident">std::path</span>::{<span class="ident">Path</span>, <span class="ident">PathBuf</span>};

<span class="attribute">#[<span class="ident">cfg</span>(<span class="ident">target_os</span> = <span class="string">&quot;linux&quot;</span>)]</span>
<span class="attribute">#[<span class="ident">cfg</span>(<span class="ident">target_os</span> <span class="op">=</span> <span class="string">&quot;linux&quot;</span>)]</span>
<span class="kw">fn</span> <span class="ident">main</span>() -&gt; () {
    <span class="kw">let</span> <span class="ident">foo</span> = <span class="bool-val">true</span> &amp;&amp; <span class="bool-val">false</span> || <span class="bool-val">true</span>;
    <span class="kw">let</span> <span class="kw">_</span>: <span class="kw-2">*const</span> () = <span class="number">0</span>;
    <span class="kw">let</span> <span class="kw">_</span> = <span class="kw-2">&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> = &amp;&amp;<span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> = <span class="kw-2">*</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="ident">foo</span> <span class="op">=</span> <span class="bool-val">true</span> <span class="op">&amp;&amp;</span> <span class="bool-val">false</span> <span class="op">|</span><span class="op">|</span> <span class="bool-val">true</span>;
    <span class="kw">let</span> <span class="kw">_</span>: <span class="kw-2">*const</span> () <span class="op">=</span> <span class="number">0</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="op">&amp;&amp;</span><span class="ident">foo</span>;
    <span class="kw">let</span> <span class="kw">_</span> <span class="op">=</span> <span class="kw-2">*</span><span class="ident">foo</span>;
    <span class="macro">mac!</span>(<span class="ident">foo</span>, <span class="kw-2">&amp;mut</span> <span class="ident">bar</span>);
    <span class="macro">assert!</span>(<span class="self">self</span>.<span class="ident">length</span> &lt; <span class="ident">N</span> &amp;&amp; <span class="ident">index</span> &lt;= <span class="self">self</span>.<span class="ident">length</span>);
    <span class="macro">assert!</span>(<span class="self">self</span>.<span class="ident">length</span> <span class="op">&lt;</span> <span class="ident">N</span> <span class="op">&amp;&amp;</span> <span class="ident">index</span> <span class="op">&lt;</span><span class="op">=</span> <span class="self">self</span>.<span class="ident">length</span>);
    <span class="ident">::std::env::var</span>(<span class="string">&quot;gateau&quot;</span>).<span class="ident">is_ok</span>();
    <span class="attribute">#[<span class="ident">rustfmt::skip</span>]</span>
    <span class="kw">let</span> <span class="ident">s</span>:<span class="ident">std::path::PathBuf</span> = <span class="ident">std::path::PathBuf::new</span>();
    <span class="kw">let</span> <span class="kw-2">mut</span> <span class="ident">s</span> = <span class="ident">String::new</span>();
    <span class="kw">let</span> <span class="ident">s</span>:<span class="ident">std::path::PathBuf</span> <span class="op">=</span> <span class="ident">std::path::PathBuf::new</span>();
    <span class="kw">let</span> <span class="kw-2">mut</span> <span class="ident">s</span> <span class="op">=</span> <span class="ident">String::new</span>();

    <span class="kw">match</span> <span class="kw-2">&amp;</span><span class="ident">s</span> {
        <span class="kw-2">ref</span> <span class="kw-2">mut</span> <span class="ident">x</span> =&gt; {}
}


<span class="macro">macro_rules!</span> <span class="ident">bar</span> {
    (<span class="macro-nonterminal">$</span><span class="macro-nonterminal">foo</span>:<span class="ident">tt</span>) =&gt; {};
}
</code></pre>

----


---
   --> fixtures/union.html

Expect:
----
<span class="kw">union</span> <span class="ident">Foo</span> {
    <span class="ident">i</span>: <span class="ident">i8</span>,
    <span class="ident">u</span>: <span class="ident">i8</span>,


<span class="kw">fn</span> <span class="ident">main</span>() {
    <span class="kw">let</span> <span class="ident">union</span> <span class="op">=</span> <span class="number">0</span>;

----

Actual:
Actual:
----
<span class="kw">union</span> <span class="ident">Foo</span> {
    <span class="ident">i</span>: <span class="ident">i8</span>,
    <span class="ident">u</span>: <span class="ident">i8</span>,


<span class="kw">fn</span> <span class="ident">main</span>() {
    <span class="kw">let</span> <span class="ident">union</span> = <span class="number">0</span>;

----

Diff:
Diff:
----
<span class="kw">union</span> <span class="ident">Foo</span> {
    <span class="ident">i</span>: <span class="ident">i8</span>,
    <span class="ident">u</span>: <span class="ident">i8</span>,


<span class="kw">fn</span> <span class="ident">main</span>() {
    <span class="kw">let</span> <span class="ident">union</span> = <span class="number">0</span>;
    <span class="kw">let</span> <span class="ident">union</span> <span class="op">=</span> <span class="number">0</span>;


----

