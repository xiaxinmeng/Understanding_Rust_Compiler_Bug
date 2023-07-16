plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
# Compile the test library with coverage instrumentation
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview  ../coverage/lib/used_inline_crate.rs \
    $( sed -n 's/^\/\/ compile-flags: \([^#]*\).*/\1/p' ../coverage/lib/used_inline_crate.rs ) \
  --crate-type rlib \
  -Ztrim-diagnostic-paths=no \
  -Zdump-mir=InstrumentCoverage -Zdump-mir-spanview -Zdump-mir-dir="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_inline_crate \
  -Zinstrument-coverage
for path in "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_inline_crate/*; do \
 file="$(basename "$path")"; \
 urlescaped="$("/usr/bin/python3" ../coverage-spanview/escape_url.py $file)" || exit $?; \
 printf "$SPANVIEW_HEADER\n" "used_inline_crate" "$urlescaped" > "$path".modified; \
 tail -n +2 "$path" >> "$path".modified; \
 mv "$path".modified "$path"; \
done && true # for/done ends in non-zero status
# Check that the selected `mir_dump` files match what we expect (`--bless` refreshes `expected`)
mkdir -p "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate
rm -f "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate/*
cp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/mir_dump.used_inline_crate/*InstrumentCoverage.0.html "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/ "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview"/actual_mir_dump.used_inline_crate/
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.unused_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.unused_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.unused_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.unused_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -73,36 +73,36 @@
 66:19-66:35: @1[0]: _3 = &amp;_4
 66:19-66:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 66:19-66:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-66:9-66:16: @2[3]: FakeRead(ForLet, _1)
+66:9-66:16: @2[3]: FakeRead(ForLet(None), _1)
 67:25-67:26: @3[2]: _5 = const 2_i32
-67:9-67:22: @3[3]: FakeRead(ForLet, _5)
+67:9-67:22: @3[3]: FakeRead(ForLet(None), _5)
 68:9-68:16: @3[6]: _7 = _1
 68:8-68:16: @3[7]: _6 = Not(move _7)"><span class="annotation">@0,1,2,3⦊</span>pub fn unused_function() {</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="66:19-66:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb8]
 66:19-66:35: @1[0]: _3 = &amp;_4
 66:19-66:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 66:19-66:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-66:9-66:16: @2[3]: FakeRead(ForLet, _1)
+66:9-66:16: @2[3]: FakeRead(ForLet(None), _1)
 67:25-67:26: @3[2]: _5 = const 2_i32
-67:9-67:22: @3[3]: FakeRead(ForLet, _5)
+67:9-67:22: @3[3]: FakeRead(ForLet(None), _5)
 68:9-68:16: @3[6]: _7 = _1
 68:8-68:16: @3[7]: _6 = Not(move _7)">    let is_true = std::env::args().len() == 1;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="66:19-66:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb8]
 66:19-66:35: @1[0]: _3 = &amp;_4
 66:19-66:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 66:19-66:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-66:9-66:16: @2[3]: FakeRead(ForLet, _1)
+66:9-66:16: @2[3]: FakeRead(ForLet(None), _1)
 67:25-67:26: @3[2]: _5 = const 2_i32
-67:9-67:22: @3[3]: FakeRead(ForLet, _5)
+67:9-67:22: @3[3]: FakeRead(ForLet(None), _5)
 68:9-68:16: @3[6]: _7 = _1
 68:8-68:16: @3[7]: _6 = Not(move _7)">    let mut countdown = 2;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="66:19-66:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb8]
 66:19-66:35: @1[0]: _3 = &amp;_4
 66:19-66:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 66:19-66:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-66:9-66:16: @2[3]: FakeRead(ForLet, _1)
+66:9-66:16: @2[3]: FakeRead(ForLet(None), _1)
 67:25-67:26: @3[2]: _5 = const 2_i32
-67:9-67:22: @3[3]: FakeRead(ForLet, _5)
+67:9-67:22: @3[3]: FakeRead(ForLet(None), _5)
 68:9-68:16: @3[6]: _7 = _1
 68:8-68:16: @3[7]: _6 = Not(move _7)">    if !is_true<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code odd" style="--layer: 1" title="69:9-69:23: @4[0]: _5 = const 20_i32
 68:17-70:6: @4[1]: _0 = const ()"><span class="annotation">@4⦊</span>{</span></span>
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.unused_generic_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.unused_generic_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.unused_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.unused_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -75,7 +75,7 @@
 61:14-61:49: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 61:51-61:54: @0[17]: _14 = &amp;_1
 61:5-61:56: @0[18]: _13 = (move _14,)
-61:5-61:56: @0[20]: FakeRead(ForMatchedPlace, _13)
+61:5-61:56: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 61:5-61:56: @0[22]: _15 = (_13.0: &amp;T)
 61:5-61:56: @0[25]: _17 = &amp;(*_15)
 61:5-61:56: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -95,7 +95,7 @@
 61:14-61:49: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 61:51-61:54: @0[17]: _14 = &amp;_1
 61:5-61:56: @0[18]: _13 = (move _14,)
-61:5-61:56: @0[20]: FakeRead(ForMatchedPlace, _13)
+61:5-61:56: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 61:5-61:56: @0[22]: _15 = (_13.0: &amp;T)
 61:5-61:56: @0[25]: _17 = &amp;(*_15)
 61:5-61:56: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -115,7 +115,7 @@
 61:14-61:49: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 61:51-61:54: @0[17]: _14 = &amp;_1
 61:5-61:56: @0[18]: _13 = (move _14,)
-61:5-61:56: @0[20]: FakeRead(ForMatchedPlace, _13)
+61:5-61:56: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 61:5-61:56: @0[22]: _15 = (_13.0: &amp;T)
 61:5-61:56: @0[25]: _17 = &amp;(*_15)
 61:5-61:56: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.unused_private_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.unused_private_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.unused_private_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.unused_private_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -73,36 +73,36 @@
 75:19-75:35: @1[0]: _3 = &amp;_4
 75:19-75:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 75:19-75:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-75:9-75:16: @2[3]: FakeRead(ForLet, _1)
+75:9-75:16: @2[3]: FakeRead(ForLet(None), _1)
 76:25-76:26: @3[2]: _5 = const 2_i32
-76:9-76:22: @3[3]: FakeRead(ForLet, _5)
+76:9-76:22: @3[3]: FakeRead(ForLet(None), _5)
 77:9-77:16: @3[6]: _7 = _1
 77:8-77:16: @3[7]: _6 = Not(move _7)"><span class="annotation">@0,1,2,3⦊</span>fn unused_private_function() {</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="75:19-75:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb8]
 75:19-75:35: @1[0]: _3 = &amp;_4
 75:19-75:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 75:19-75:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-75:9-75:16: @2[3]: FakeRead(ForLet, _1)
+75:9-75:16: @2[3]: FakeRead(ForLet(None), _1)
 76:25-76:26: @3[2]: _5 = const 2_i32
-76:9-76:22: @3[3]: FakeRead(ForLet, _5)
+76:9-76:22: @3[3]: FakeRead(ForLet(None), _5)
 77:9-77:16: @3[6]: _7 = _1
 77:8-77:16: @3[7]: _6 = Not(move _7)">    let is_true = std::env::args().len() == 1;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="75:19-75:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb8]
 75:19-75:35: @1[0]: _3 = &amp;_4
 75:19-75:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 75:19-75:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-75:9-75:16: @2[3]: FakeRead(ForLet, _1)
+75:9-75:16: @2[3]: FakeRead(ForLet(None), _1)
 76:25-76:26: @3[2]: _5 = const 2_i32
-76:9-76:22: @3[3]: FakeRead(ForLet, _5)
+76:9-76:22: @3[3]: FakeRead(ForLet(None), _5)
 77:9-77:16: @3[6]: _7 = _1
 77:8-77:16: @3[7]: _6 = Not(move _7)">    let mut countdown = 2;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="75:19-75:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb8]
 75:19-75:35: @1[0]: _3 = &amp;_4
 75:19-75:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb7]
 75:19-75:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-75:9-75:16: @2[3]: FakeRead(ForLet, _1)
+75:9-75:16: @2[3]: FakeRead(ForLet(None), _1)
 76:25-76:26: @3[2]: _5 = const 2_i32
-76:9-76:22: @3[3]: FakeRead(ForLet, _5)
+76:9-76:22: @3[3]: FakeRead(ForLet(None), _5)
 77:9-77:16: @3[6]: _7 = _1
 77:8-77:16: @3[7]: _6 = Not(move _7)">    if !is_true<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code odd" style="--layer: 1" title="78:9-78:23: @4[0]: _5 = const 20_i32
 77:17-79:6: @4[1]: _0 = const ()"><span class="annotation">@4⦊</span>{</span></span>
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.use_this_lib_crate.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.use_this_lib_crate.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.use_this_lib_crate.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.use_this_lib_crate.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -76,7 +76,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -89,7 +89,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -102,7 +102,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -115,7 +115,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -128,7 +128,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -141,7 +141,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -154,7 +154,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -167,7 +167,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
@@ -180,7 +180,7 @@
 87:20-87:36: @2[7]: _5 = move _6
 87:20-87:36: @2[8]: _4 = move _5 as std::boxed::Box&lt;[i32]&gt; (Pointer(Unsize))
 87:20-87:36: @4.Call: _3 = std::slice::&lt;impl [i32]&gt;::into_vec::&lt;std::alloc::Global&gt;(move _4) -&gt; [return: bb5, unwind: bb12]
-87:9-87:17: @5[1]: FakeRead(ForLet, _3)
+87:9-87:17: @5[1]: FakeRead(ForLet(None), _3)
 88:52-88:60: @5[4]: _8 = move _3
 88:5-88:61: @5.Call: _7 = used_only_from_this_lib_crate_generic_function::&lt;std::vec::Vec&lt;i32&gt;&gt;(move _8) -&gt; [return: bb6, unwind: bb9]
 89:5-89:91: @6.Call: _9 = used_only_from_this_lib_crate_generic_function::&lt;&amp;str&gt;(const &quot;used ONLY from library used_crate.rs&quot;) -&gt; [return: bb7, unwind: bb10]
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.used_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.used_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -75,7 +75,7 @@
 51:14-51:76: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 51:78-51:81: @0[17]: _14 = &amp;_1
 51:5-51:83: @0[18]: _13 = (move _14,)
-51:5-51:83: @0[20]: FakeRead(ForMatchedPlace, _13)
+51:5-51:83: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 51:5-51:83: @0[22]: _15 = (_13.0: &amp;T)
 51:5-51:83: @0[25]: _17 = &amp;(*_15)
 51:5-51:83: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -95,7 +95,7 @@
 51:14-51:76: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 51:78-51:81: @0[17]: _14 = &amp;_1
 51:5-51:83: @0[18]: _13 = (move _14,)
-51:5-51:83: @0[20]: FakeRead(ForMatchedPlace, _13)
+51:5-51:83: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 51:5-51:83: @0[22]: _15 = (_13.0: &amp;T)
 51:5-51:83: @0[25]: _17 = &amp;(*_15)
 51:5-51:83: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -115,7 +115,7 @@
 51:14-51:76: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 51:78-51:81: @0[17]: _14 = &amp;_1
 51:5-51:83: @0[18]: _13 = (move _14,)
-51:5-51:83: @0[20]: FakeRead(ForMatchedPlace, _13)
+51:5-51:83: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 51:5-51:83: @0[22]: _15 = (_13.0: &amp;T)
 51:5-51:83: @0[25]: _17 = &amp;(*_15)
 51:5-51:83: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.used_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.used_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -77,25 +77,25 @@
 11:19-11:35: @1[0]: _3 = &amp;_4
 11:19-11:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb8]
 11:19-11:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-11:9-11:16: @2[3]: FakeRead(ForLet, _1)
+11:9-11:16: @2[3]: FakeRead(ForLet(None), _1)
 12:25-12:26: @3[2]: _5 = const 0_i32
-12:9-12:22: @3[3]: FakeRead(ForLet, _5)
+12:9-12:22: @3[3]: FakeRead(ForLet(None), _5)
 13:8-13:15: @3[6]: _7 = _1"><span class="annotation">@0,1,2,3⦊</span>is_true = std::env::args().len() == 1;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="11:19-11:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb9]
 11:19-11:35: @1[0]: _3 = &amp;_4
 11:19-11:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb8]
 11:19-11:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-11:9-11:16: @2[3]: FakeRead(ForLet, _1)
+11:9-11:16: @2[3]: FakeRead(ForLet(None), _1)
 12:25-12:26: @3[2]: _5 = const 0_i32
-12:9-12:22: @3[3]: FakeRead(ForLet, _5)
+12:9-12:22: @3[3]: FakeRead(ForLet(None), _5)
 13:8-13:15: @3[6]: _7 = _1">    let mut countdown = 0;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="11:19-11:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb9]
 11:19-11:35: @1[0]: _3 = &amp;_4
 11:19-11:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb8]
 11:19-11:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-11:9-11:16: @2[3]: FakeRead(ForLet, _1)
+11:9-11:16: @2[3]: FakeRead(ForLet(None), _1)
 12:25-12:26: @3[2]: _5 = const 0_i32
-12:9-12:22: @3[3]: FakeRead(ForLet, _5)
+12:9-12:22: @3[3]: FakeRead(ForLet(None), _5)
 13:8-13:15: @3[6]: _7 = _1">    if is_true<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code odd" style="--layer: 1" title="14:9-14:23: @4[0]: _5 = const 10_i32
 13:16-15:6: @4[1]: _6 = const ()"><span class="annotation">@4⦊</span>{</span></span>
 <span class="line"><span class="code odd" style="--layer: 1" title="14:9-14:23: @4[0]: _5 = const 10_i32
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.used_inline_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_inline_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.used_inline_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_inline_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -77,25 +77,25 @@
 24:19-24:35: @1[0]: _3 = &amp;_4
 24:19-24:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb8]
 24:19-24:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-24:9-24:16: @2[3]: FakeRead(ForLet, _1)
+24:9-24:16: @2[3]: FakeRead(ForLet(None), _1)
 25:25-25:26: @3[2]: _5 = const 0_i32
-25:9-25:22: @3[3]: FakeRead(ForLet, _5)
+25:9-25:22: @3[3]: FakeRead(ForLet(None), _5)
 26:8-26:15: @3[6]: _7 = _1"><span class="annotation">@0,1,2,3⦊</span>is_true = std::env::args().len() == 1;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="24:19-24:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb9]
 24:19-24:35: @1[0]: _3 = &amp;_4
 24:19-24:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb8]
 24:19-24:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-24:9-24:16: @2[3]: FakeRead(ForLet, _1)
+24:9-24:16: @2[3]: FakeRead(ForLet(None), _1)
 25:25-25:26: @3[2]: _5 = const 0_i32
-25:9-25:22: @3[3]: FakeRead(ForLet, _5)
+25:9-25:22: @3[3]: FakeRead(ForLet(None), _5)
 26:8-26:15: @3[6]: _7 = _1">    let mut countdown = 0;</span></span>
 <span class="line"><span class="code even" style="--layer: 1" title="24:19-24:35: @0.Call: _4 = std::env::args() -&gt; [return: bb1, unwind: bb9]
 24:19-24:35: @1[0]: _3 = &amp;_4
 24:19-24:41: @1.Call: _2 = &lt;std::env::Args as std::iter::ExactSizeIterator&gt;::len(move _3) -&gt; [return: bb2, unwind: bb8]
 24:19-24:46: @2[1]: _1 = Eq(move _2, const 1_usize)
-24:9-24:16: @2[3]: FakeRead(ForLet, _1)
+24:9-24:16: @2[3]: FakeRead(ForLet(None), _1)
 25:25-25:26: @3[2]: _5 = const 0_i32
-25:9-25:22: @3[3]: FakeRead(ForLet, _5)
+25:9-25:22: @3[3]: FakeRead(ForLet(None), _5)
 26:8-26:15: @3[6]: _7 = _1">    if is_true<span class="annotation">⦉@0,1,2,3</span></span></span><span class="code" style="--layer: 0"> </span><span><span class="code odd" style="--layer: 1" title="27:9-27:23: @4[0]: _5 = const 10_i32
 26:16-28:6: @4[1]: _6 = const ()"><span class="annotation">@4⦊</span>{</span></span>
 <span class="line"><span class="code odd" style="--layer: 1" title="27:9-27:23: @4[0]: _5 = const 10_i32
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.used_only_from_bin_crate_generic_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_only_from_bin_crate_generic_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.used_only_from_bin_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_only_from_bin_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -75,7 +75,7 @@
 40:14-40:67: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 40:69-40:72: @0[17]: _14 = &amp;_1
 40:5-40:74: @0[18]: _13 = (move _14,)
-40:5-40:74: @0[20]: FakeRead(ForMatchedPlace, _13)
+40:5-40:74: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 40:5-40:74: @0[22]: _15 = (_13.0: &amp;T)
 40:5-40:74: @0[25]: _17 = &amp;(*_15)
 40:5-40:74: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -95,7 +95,7 @@
 40:14-40:67: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 40:69-40:72: @0[17]: _14 = &amp;_1
 40:5-40:74: @0[18]: _13 = (move _14,)
-40:5-40:74: @0[20]: FakeRead(ForMatchedPlace, _13)
+40:5-40:74: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 40:5-40:74: @0[22]: _15 = (_13.0: &amp;T)
 40:5-40:74: @0[25]: _17 = &amp;(*_15)
 40:5-40:74: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -115,7 +115,7 @@
 40:14-40:67: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 40:69-40:72: @0[17]: _14 = &amp;_1
 40:5-40:74: @0[18]: _13 = (move _14,)
-40:5-40:74: @0[20]: FakeRead(ForMatchedPlace, _13)
+40:5-40:74: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 40:5-40:74: @0[22]: _15 = (_13.0: &amp;T)
 40:5-40:74: @0[25]: _17 = &amp;(*_15)
 40:5-40:74: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.used_only_from_this_lib_crate_generic_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_only_from_this_lib_crate_generic_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.used_only_from_this_lib_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_only_from_this_lib_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -75,7 +75,7 @@
 46:14-46:72: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 46:74-46:77: @0[17]: _14 = &amp;_1
 46:5-46:79: @0[18]: _13 = (move _14,)
-46:5-46:79: @0[20]: FakeRead(ForMatchedPlace, _13)
+46:5-46:79: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 46:5-46:79: @0[22]: _15 = (_13.0: &amp;T)
 46:5-46:79: @0[25]: _17 = &amp;(*_15)
 46:5-46:79: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -95,7 +95,7 @@
 46:14-46:72: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 46:74-46:77: @0[17]: _14 = &amp;_1
 46:5-46:79: @0[18]: _13 = (move _14,)
-46:5-46:79: @0[20]: FakeRead(ForMatchedPlace, _13)
+46:5-46:79: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 46:5-46:79: @0[22]: _15 = (_13.0: &amp;T)
 46:5-46:79: @0[25]: _17 = &amp;(*_15)
 46:5-46:79: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -115,7 +115,7 @@
 46:14-46:72: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 46:74-46:77: @0[17]: _14 = &amp;_1
 46:5-46:79: @0[18]: _13 = (move _14,)
-46:5-46:79: @0[20]: FakeRead(ForMatchedPlace, _13)
+46:5-46:79: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 46:5-46:79: @0[22]: _15 = (_13.0: &amp;T)
 46:5-46:79: @0[25]: _17 = &amp;(*_15)
 46:5-46:79: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
diff -u --strip-trailing-cr -r expected_mir_dump.used_inline_crate/used_inline_crate.used_with_same_type_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_with_same_type_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html
--- expected_mir_dump.used_inline_crate/used_inline_crate.used_with_same_type_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:04:36.450659247 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/coverage-spanview/coverage-spanview/actual_mir_dump.used_inline_crate/used_inline_crate.used_with_same_type_from_bin_crate_and_lib_crate_generic_function.-------.InstrumentCoverage.0.html 2021-03-31 23:46:43.963687435 +0000
@@ -75,7 +75,7 @@
 56:14-56:91: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 56:93-56:96: @0[17]: _14 = &amp;_1
 56:5-56:98: @0[18]: _13 = (move _14,)
-56:5-56:98: @0[20]: FakeRead(ForMatchedPlace, _13)
+56:5-56:98: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 56:5-56:98: @0[22]: _15 = (_13.0: &amp;T)
 56:5-56:98: @0[25]: _17 = &amp;(*_15)
 56:5-56:98: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -95,7 +95,7 @@
 56:14-56:91: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 56:93-56:96: @0[17]: _14 = &amp;_1
 56:5-56:98: @0[18]: _13 = (move _14,)
-56:5-56:98: @0[20]: FakeRead(ForMatchedPlace, _13)
+56:5-56:98: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 56:5-56:98: @0[22]: _15 = (_13.0: &amp;T)
 56:5-56:98: @0[25]: _17 = &amp;(*_15)
 56:5-56:98: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
@@ -115,7 +115,7 @@
 56:14-56:91: @0[9]: _5 = move _6 as &amp;[&amp;str] (Pointer(Unsize))
 56:93-56:96: @0[17]: _14 = &amp;_1
 56:5-56:98: @0[18]: _13 = (move _14,)
-56:5-56:98: @0[20]: FakeRead(ForMatchedPlace, _13)
+56:5-56:98: @0[20]: FakeRead(ForMatchedPlace(None), _13)
 56:5-56:98: @0[22]: _15 = (_13.0: &amp;T)
 56:5-56:98: @0[25]: _17 = &amp;(*_15)
 56:5-56:98: @0[27]: _18 = &lt;T as std::fmt::Debug&gt;::fmt as for&lt;&#39;r, &#39;s, &#39;t0&gt; fn(&amp;&#39;r T, &amp;&#39;s mut std::fmt::Formatter&lt;&#39;t0&gt;) -&gt; std::result::Result&lt;(), std::fmt::Error&gt; (Pointer(ReifyFnPointer))
------------------------------------------
stderr:
------------------------------------------
warning: function is never used: `unused_private_function`
---
test result: FAILED. 215 passed; 1 failed; 6 ignored; 0 measured; 0 filtered out; finished in 29.18s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:40:07
