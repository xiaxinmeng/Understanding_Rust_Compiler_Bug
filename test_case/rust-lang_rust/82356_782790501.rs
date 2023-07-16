plain
.................................................................................................... 9200/11472
.................................................................................................... 9300/11472
.................................................................................................... 9400/11472
.............................i......i............................................................... 9500/11472
....................................................................iiiiiii..iiiiii.i............... 9600/11472
.................................................................................................... 9800/11472
.................................................................................................... 9900/11472
.................................................................................................... 10000/11472
.................................................................................................... 10100/11472
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.075 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.718 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.410 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; generated diffs will be harder to read

running 420 tests
.FFF.FFF.FF.FFFF.F..FFFF.FFFFFFF...FiFFFFFFFF.F..FF..FFFFF.F..FFFFFFF..FFFFFFF.FFFFFFFFFFFFFFFF.FFFF 100/420
FF.FFF.FFFFFFFFFFFFFF.FFFFFFFFFFFF.FFFFF.F..F.FFF.F.FF.F..FF.FF.FFFFFFF.F.FF.F.FF.FF.F.FF.FFFFF.F.FF 200/420
FF.FFF.F.F..FFFFFFFFF..F..F.FFF.FF..F.F.FFF.FFFFFFFF.F.F..F.......F.F..FF.FF.FF.F.F.F..F.FFF.FFFFFF. 300/420
F....F.F.F.F.FFFFFFFF....F..FFF.FFFFFFiFFF.F..F.FFF.F..F.FFFF.F.FFFFFFF.FFFFF.FFFF.FFF.F.FFFFFFFFFFF 400/420
FFFFFFFFFF.FFFFFFFFF

---- [rustdoc] rustdoc/auto_aliases.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto_aliases" "/checkout/src/test/rustdoc/auto_aliases.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'auto_aliases/trait.Bar.html'
 // @has auto_aliases/trait.Bar.html '//h3[@aliases="auto_aliases::Foo"]' 'impl Bar for Foo'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/auto_aliases.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto_aliases/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto_aliases.nightly" "/checkout/src/test/rustdoc/auto_aliases.rs"`', src/tools/compiletest/src/runtest.rs:1871:33

---- [rustdoc] rustdoc/assoc-item-cast.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast" "/checkout/src/test/rustdoc/assoc-item-cast.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
14: @has check failed
 File does not exist 'foo/type.AsExprOf.html'
 // @has foo/type.AsExprOf.html
15: @has check failed
 File does not exist 'foo/type.AsExprOf.html'
 // @has - '//*[@class="rust typedef"]' 'type AsExprOf<Item, Type> = <Item as AsExpression<Type>>::Expression;'
Encountered 2 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/assoc-item-cast.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast.nightly" "/checkout/src/test/rustdoc/assoc-item-cast.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/bad-codeblock-syntax.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/bad-codeblock-syntax" "/checkout/src/test/rustdoc/bad-codeblock-syntax.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
1: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.foo.html'
 // @has bad_codeblock_syntax/fn.foo.html
2: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.foo.html'
 // @has - '//*[@class="docblock"]' '\_'
8: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.bar.html'
 // @has bad_codeblock_syntax/fn.bar.html
9: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.bar.html'
 // @has - '//*[@class="docblock"]' '`baz::foobar`'
15: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.quux.html'
 // @has bad_codeblock_syntax/fn.quux.html
16: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.quux.html'
 // @has - '//*[@class="docblock"]' '\_'
22: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.ok.html'
 // @has bad_codeblock_syntax/fn.ok.html
23: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.ok.html'
 // @has - '//*[@class="docblock"]' '\_'
29: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.escape.html'
 // @has bad_codeblock_syntax/fn.escape.html
30: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.escape.html'
 // @has - '//*[@class="docblock"]' '\_ <script>alert("not valid Rust");</script>'
37: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.unterminated.html'
 // @has bad_codeblock_syntax/fn.unterminated.html
38: @has check failed
 File does not exist 'bad_codeblock_syntax/fn.unterminated.html'
 // @has - '//*[@class="docblock"]' '"unterminated'
Encountered 12 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/bad-codeblock-syntax.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/bad-codeblock-syntax/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/bad-codeblock-syntax.nightly" "/checkout/src/test/rustdoc/bad-codeblock-syntax.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/assoc-consts.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
2: @has check failed
 File does not exist 'assoc_consts/trait.Foo.html'
     // @has assoc_consts/trait.Foo.html '//*[@class="rust trait"]' 'const FOO: usize;'
4: @has check failed
 File does not exist 'assoc_consts/trait.Foo.html'
     // @has - '//*[@id="associatedconstant.FOO"]' 'const FOO: usize'
6: @has check failed
 File does not exist 'assoc_consts/trait.Foo.html'
     // @has - '//*[@id="associatedconstant.FOO_NO_DEFAULT"]' 'const FOO_NO_DEFAULT: bool'
8: @!has check failed
 File does not exist 'assoc_consts/trait.Foo.html'
     // @!has - FOO_HIDDEN
16: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has assoc_consts/struct.Bar.html '//code' 'impl Foo for Bar'
17: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@id="associatedconstant.FOO"]' 'const FOO: usize'
19: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@id="associatedconstant.FOO_NO_DEFAULT"]' 'const FOO_NO_DEFAULT: bool'
21: @!has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @!has - FOO_HIDDEN
27: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.BAR"]' 'const BAR: usize'
35: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.BAZ"]' "const BAZ: Baz<'static, u8, u32>"
43: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has assoc_consts/struct.Bar.html '//*[@id="associatedconstant.F"]' "const F: fn(_: &(dyn ToString + 'static))"
49: @!has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @!has assoc_consts/struct.Bar.html 'BAR_PRIVATE'
51: @!has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @!has assoc_consts/struct.Bar.html 'BAR_HIDDEN'
56: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
 // @has assoc_consts/trait.Qux.html
58: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@id="associatedconstant.QUX0"]' 'const QUX0: u8'
59: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX0 in trait."
62: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@id="associatedconstant.QUX1"]' 'const QUX1: i8'
63: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX1 in trait."
66: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@id="associatedconstant.QUX_DEFAULT0"]' 'const QUX_DEFAULT0: u16'
67: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT12 in trait."
70: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@id="associatedconstant.QUX_DEFAULT1"]' 'const QUX_DEFAULT1: i16'
71: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT1 in trait."
74: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@id="associatedconstant.QUX_DEFAULT2"]' 'const QUX_DEFAULT2: u32'
75: @has check failed
 File does not exist 'assoc_consts/trait.Qux.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT2 in trait."
80: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
 // @has assoc_consts/struct.Bar.html '//code' 'impl Qux for Bar'
82: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@id="associatedconstant.QUX0"]' 'const QUX0: u8'
83: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX0 in trait."
86: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@id="associatedconstant.QUX1"]' 'const QUX1: i8'
87: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX1 in impl."
90: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@id="associatedconstant.QUX_DEFAULT0"]' 'const QUX_DEFAULT0: u16'
91: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@class="docblock hidden"]' "Docs for QUX_DEFAULT12 in trait."
93: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@id="associatedconstant.QUX_DEFAULT1"]' 'const QUX_DEFAULT1: i16'
94: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT1 in impl."
97: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@id="associatedconstant.QUX_DEFAULT2"]' 'const QUX_DEFAULT2: u32'
98: @has check failed
 File does not exist 'assoc_consts/struct.Bar.html'
     // @has - '//*[@class="docblock hidden"]' "Docs for QUX_DEFAULT2 in trait."
Encountered 35 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/assoc-consts.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts.nightly" "/checkout/src/test/rustdoc/assoc-consts.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/async-fn.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'async_fn/fn.foo.html'
 // @has async_fn/fn.foo.html '//pre[@class="rust fn"]' 'pub async fn foo() -> Option<Foo>'
8: @has check failed
 File does not exist 'async_fn/fn.bar.html'
 // @has async_fn/fn.bar.html '//pre[@class="rust fn"]' 'pub async fn bar(a: i32, b: i32) -> i32'
13: @has check failed
 File does not exist 'async_fn/fn.baz.html'
 // @has async_fn/fn.baz.html '//pre[@class="rust fn"]' 'pub async fn baz<T>(a: T) -> T'
18: @has check failed
 File does not exist 'async_fn/fn.qux.html'
 // @has async_fn/fn.qux.html '//pre[@class="rust fn"]' 'pub async unsafe fn qux() -> char'
23: @has check failed
 File does not exist 'async_fn/fn.mut_args.html'
 // @has async_fn/fn.mut_args.html '//pre[@class="rust fn"]' 'pub async fn mut_args(a: usize)'
26: @has check failed
 File does not exist 'async_fn/fn.mut_ref.html'
 // @has async_fn/fn.mut_ref.html '//pre[@class="rust fn"]' 'pub async fn mut_ref(x: i32)'
33: @has check failed
 File does not exist 'async_fn/fn.quux.html'
 // @has async_fn/fn.quux.html '//pre[@class="rust fn"]' 'pub async fn quux() -> impl Bar'
38: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @has async_fn/struct.Foo.html
39: @matches check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @matches - '//code' 'pub async fn f\(\)$'
40: @matches check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @matches - '//code' 'pub async unsafe fn g\(\)$'
41: @matches check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @matches - '//code' 'pub async fn mut_self\(self, first: usize\)$'
53: @has check failed
 File does not exist 'async_fn/fn.const_generics.html'
 // @has async_fn/fn.const_generics.html
54: @has check failed
 File does not exist 'async_fn/fn.const_generics.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn const_generics<const N: usize>(_: impl Trait<N>)'
59: @has check failed
 File does not exist 'async_fn/fn.elided.html'
 // @has async_fn/fn.elided.html
60: @has check failed
 File does not exist 'async_fn/fn.elided.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn elided(foo: &str) -> &str'
64: @has check failed
 File does not exist 'async_fn/fn.user_elided.html'
 // @has async_fn/fn.user_elided.html
65: @has check failed
 File does not exist 'async_fn/fn.user_elided.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn user_elided(foo: &str) -> &str'
67: @has check failed
 File does not exist 'async_fn/fn.static_trait.html'
 // @has async_fn/fn.static_trait.html
68: @has check failed
 File does not exist 'async_fn/fn.static_trait.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn static_trait(foo: &str) -> Box<dyn Bar>'
70: @has check failed
 File does not exist 'async_fn/fn.lifetime_for_trait.html'
 // @has async_fn/fn.lifetime_for_trait.html
71: @has check failed
 File does not exist 'async_fn/fn.lifetime_for_trait.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn lifetime_for_trait(foo: &str) -> Box<dyn Bar + '_>"
73: @has check failed
 File does not exist 'async_fn/fn.elided_in_input_trait.html'
 // @has async_fn/fn.elided_in_input_trait.html
74: @has check failed
 File does not exist 'async_fn/fn.elided_in_input_trait.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn elided_in_input_trait(t: impl Pattern<'_>)"
80: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has async_fn/struct.Foo.html
81: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has - '//h4[@class="method"]' 'pub async fn complicated_lifetimes( &self, context: &impl Bar) -> impl Iterator<Item = &usize>'
84: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has - '//h4[@class="method"]' "pub async fn readable<T>(&self) -> Result<AsyncFdReadyGuard<'_, T>, ()>"
86: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has - '//h4[@class="method"]' "pub async fn mut_self(&mut self)"
91: @has check failed
 File does not exist 'async_fn/fn.named.html'
 // @has async_fn/fn.named.html
92: @has check failed
 File does not exist 'async_fn/fn.named.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn named<'a, 'b>(foo: &'a str) -> &'b str"
94: @has check failed
 File does not exist 'async_fn/fn.named_trait.html'
 // @has async_fn/fn.named_trait.html
95: @has check failed
 File does not exist 'async_fn/fn.named_trait.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn named_trait<'a, 'b>(foo: impl Pattern<'a>) -> impl Pattern<'b>"
Encountered 31 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/async-fn.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn.nightly" "/checkout/src/test/rustdoc/async-fn.rs" "--edition=2018"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/cap-lints.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cap-lints" "/checkout/src/test/rustdoc/cap-lints.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
6: @has check failed
 File does not exist 'cap_lints/struct.Foo.html'
 // @has cap_lints/struct.Foo.html //pre '#[must_use]'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/cap-lints.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cap-lints/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cap-lints.nightly" "/checkout/src/test/rustdoc/cap-lints.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/assoc-consts-version.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version" "/checkout/src/test/rustdoc/assoc-consts-version.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
11: @has check failed
 File does not exist 'foo/struct.SomeStruct.html'
     // @has 'foo/struct.SomeStruct.html' '//*[@id="associatedconstant.SOME_CONST"]//span[@class="since"]' '1.1.2'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/assoc-consts-version.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version.nightly" "/checkout/src/test/rustdoc/assoc-consts-version.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/blanket-reexport-item.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'foo/struct.S.html'
 // @has foo/struct.S.html '//h3[@id="impl-Into%3CU%3E"]//code' 'impl<T, U> Into<U> for T'
Encountered 1 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/blanket-reexport-item.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/blanket-reexport-item.nightly" "/checkout/src/test/rustdoc/blanket-reexport-item.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/attributes.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/attributes" "/checkout/src/test/rustdoc/attributes.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'foo/fn.f.html'
 // @has foo/fn.f.html '//*[@class="docblock attributes"]' '#[no_mangle]'
7: @has check failed
 File does not exist 'foo/fn.g.html'
 // @has foo/fn.g.html '//*[@class="docblock attributes"]' '#[export_name = "bar"]'
11: @matches check failed
 File does not exist 'foo/enum.Foo.html'
 // @matches foo/enum.Foo.html '//*[@class="docblock attributes top-attr"]' '(?m)\A#\[repr\(i64\)\]\n#\[must_use\]\Z'
19: @has check failed
 File does not exist 'foo/struct.Repr.html'
 // @has foo/struct.Repr.html '//*[@class="docblock attributes top-attr"]' '#[repr(C, align(8))]'
Encountered 4 errors

------------------------------------------


info: generating a diff against nightly rustdoc
thread '[rustdoc] rustdoc/attributes.rs' panicked at 'failed to exec `"rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/attributes/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/attributes.nightly" "/checkout/src/test/rustdoc/attributes.rs"`', src/tools/compiletest/src/runtest.rs:1871:33
---- [rustdoc] rustdoc/assoc-types.rs stdout ----


error: htmldocck failed!
status: exit code: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
 // @has assoc_types/trait.Index.html
7: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
     // @has - '//*[@id="associatedtype.Output"]//code' 'type Output: ?Sized'
9: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
     // @has - '//*[@id="tymethod.index"]//code' "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
11: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
     // @has - '//*[@id="tymethod.index"]//code//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' "Output"
16: @has check failed
 File does not exist 'assoc_types/fn.use_output.html'
 // @has assoc_types/fn.use_output.html
17: @has check failed
 File does not exist 'assoc_types/fn.use_output.html'
 // @has - '//*[@class="rust fn"]' '-> &T::Output'
18: @has check failed
 File does not exist 'assoc_types/fn.use_output.html'
 // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Index.html#associatedtype.Output"]' 'Output'
27: @has check failed
 File does not exist 'assoc_types/fn.use_input.html'
 // @has assoc_types/fn.use_input.html
28: @has check failed
 File does not exist 'assoc_types/fn.use_input.html'
 // @has - '//*[@class="rust fn"]' 'T::Input'
29: @has check failed
 File does not exist 'assoc_types/fn.use_input.html'
 // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Feed.html#associatedtype.Input"]' 'Input'
32: @has check failed
 File does not exist 'assoc_types/fn.cmp_input.html'
 // @has assoc_types/fn.cmp_input.html
33: @has check failed
 File does not exist 'assoc_types/fn.cmp_input.html'
 // @has - '//*[@class="rust fn"]' 'where T::Input: PartialEq<U::Input>'
34: @has check failed
 File does not exist 'assoc_types/fn.cmp_input.html'
 // @has - '//*[@class="rust fn"]//a[@href="../assoc_types/trait.Feed.html#associatedtype.Input"]' 'Input'
Encountered 13 errors

------------------------------------------
---
test result: FAILED. 119 passed; 299 failed; 2 ignored; 0 measured; 0 filtered out; finished in 30.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:24
