plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 465 tests
iFFF.FF.FFFFFFFF.FFFFFF.FFF.FFFFF.FFFFFFF.FFiFFFFFFFFFFFFF.FFFFF.FF..F..FFFFFFFFFFFFFFFFFFFFFFFFFFFF 100/465
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.F...FFFFFFF.FF.FFFFFFFFF.FFFF.FFFFFFF. 200/465
FF.FFFFFFFF.FFFFFF.FFFF..FFFFFFFFF.FFF.FF.FFFFF.FF.FF.FFFF.F.FFFFF..FFF.FFF.FF.FF...FFF.FF..FF.FF..F 300/465
FFFF.FFFFFFF.F.FFFFF..FF.FFFFFFFFFFFFFFFFFFFFFF.FFF.FF.FF.FFFF...FFFF.FiFFFFFFF.FFFFFFFFFFFFFFFFFFFF 400/465
FFFFFFFFFFFFF.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF

---- [rustdoc] rustdoc/assoc-consts-version.rs stdout ----


error: htmldocck failed!
status: exit status: 1
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



---- [rustdoc] rustdoc/assoc-item-cast.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-item-cast" "/checkout/src/test/rustdoc/assoc-item-cast.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
13: @has check failed
 File does not exist 'foo/type.AsExprOf.html'
 // @has foo/type.AsExprOf.html
14: @has check failed
 File does not exist 'foo/type.AsExprOf.html'
 // @has - '//*[@class="rust typedef"]' 'type AsExprOf<Item, Type> = <Item as AsExpression<Type>>::Expression;'
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/asm-foreign.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/asm-foreign" "/checkout/src/test/rustdoc/asm-foreign.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @has check failed
 File does not exist 'asm_foreign/fn.aarch64.html'
 // @has asm_foreign/fn.aarch64.html
15: @has check failed
 File does not exist 'asm_foreign/fn.x86.html'
 // @has asm_foreign/fn.x86.html
Encountered 2 errors

------------------------------------------



---- [rustdoc] rustdoc/bad-codeblock-syntax.rs stdout ----

error: htmldocck failed!
status: exit status: 1
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



---- [rustdoc] rustdoc/assoc-consts.rs stdout ----

error: htmldocck failed!
status: exit status: 1
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
     // @has assoc_consts/struct.Bar.html '//h3[@class="code-header in-band"]' 'impl Foo for Bar'
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
 // @has assoc_consts/struct.Bar.html '//h3[@class="code-header in-band"]' 'impl Qux for Bar'
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
     // @has - '//div[@class="impl-items"]//*[@class="docblock"]' "Docs for QUX_DEFAULT12 in trait."
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
     // @has - '//div[@class="impl-items"]//*[@class="docblock"]' "Docs for QUX_DEFAULT2 in trait."
Encountered 35 errors

------------------------------------------



---- [rustdoc] rustdoc/async-fn.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-fn" "/checkout/src/test/rustdoc/async-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
2: @has check failed
 File does not exist 'async_fn/fn.foo.html'
 // @has async_fn/fn.foo.html '//pre[@class="rust fn"]' 'pub async fn foo() -> Option<Foo>'
7: @has check failed
 File does not exist 'async_fn/fn.bar.html'
 // @has async_fn/fn.bar.html '//pre[@class="rust fn"]' 'pub async fn bar(a: i32, b: i32) -> i32'
12: @has check failed
 File does not exist 'async_fn/fn.baz.html'
 // @has async_fn/fn.baz.html '//pre[@class="rust fn"]' 'pub async fn baz<T>(a: T) -> T'
17: @has check failed
 File does not exist 'async_fn/fn.qux.html'
 // @has async_fn/fn.qux.html '//pre[@class="rust fn"]' 'pub async unsafe fn qux() -> char'
22: @has check failed
 File does not exist 'async_fn/fn.mut_args.html'
 // @has async_fn/fn.mut_args.html '//pre[@class="rust fn"]' 'pub async fn mut_args(a: usize)'
25: @has check failed
 File does not exist 'async_fn/fn.mut_ref.html'
 // @has async_fn/fn.mut_ref.html '//pre[@class="rust fn"]' 'pub async fn mut_ref(x: i32)'
32: @has check failed
 File does not exist 'async_fn/fn.quux.html'
 // @has async_fn/fn.quux.html '//pre[@class="rust fn"]' 'pub async fn quux() -> impl Bar'
37: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @has async_fn/struct.Foo.html
38: @matches check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @matches - '//h4[@class="code-header"]' 'pub async fn f\(\)$'
39: @matches check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @matches - '//h4[@class="code-header"]' 'pub async unsafe fn g\(\)$'
40: @matches check failed
 File does not exist 'async_fn/struct.Foo.html'
 // @matches - '//h4[@class="code-header"]' 'pub async fn mut_self\(self, first: usize\)$'
52: @has check failed
 File does not exist 'async_fn/fn.const_generics.html'
 // @has async_fn/fn.const_generics.html
53: @has check failed
 File does not exist 'async_fn/fn.const_generics.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn const_generics<const N: usize>(_: impl Trait<N>)'
58: @has check failed
 File does not exist 'async_fn/fn.elided.html'
 // @has async_fn/fn.elided.html
59: @has check failed
 File does not exist 'async_fn/fn.elided.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn elided(foo: &str) -> &str'
63: @has check failed
 File does not exist 'async_fn/fn.user_elided.html'
 // @has async_fn/fn.user_elided.html
64: @has check failed
 File does not exist 'async_fn/fn.user_elided.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn user_elided(foo: &str) -> &str'
66: @has check failed
 File does not exist 'async_fn/fn.static_trait.html'
 // @has async_fn/fn.static_trait.html
67: @has check failed
 File does not exist 'async_fn/fn.static_trait.html'
 // @has - '//pre[@class="rust fn"]' 'pub async fn static_trait(foo: &str) -> Box<dyn Bar>'
69: @has check failed
 File does not exist 'async_fn/fn.lifetime_for_trait.html'
 // @has async_fn/fn.lifetime_for_trait.html
70: @has check failed
 File does not exist 'async_fn/fn.lifetime_for_trait.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn lifetime_for_trait(foo: &str) -> Box<dyn Bar + '_>"
72: @has check failed
 File does not exist 'async_fn/fn.elided_in_input_trait.html'
 // @has async_fn/fn.elided_in_input_trait.html
73: @has check failed
 File does not exist 'async_fn/fn.elided_in_input_trait.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn elided_in_input_trait(t: impl Pattern<'_>)"
79: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has async_fn/struct.Foo.html
80: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has - '//div[@class="method has-srclink"]' 'pub async fn complicated_lifetimes( &self, context: &impl Bar) -> impl Iterator<Item = &usize>'
83: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has - '//div[@class="method has-srclink"]' "pub async fn readable<T>(&self) -> Result<AsyncFdReadyGuard<'_, T>, ()>"
85: @has check failed
 File does not exist 'async_fn/struct.Foo.html'
     // @has - '//div[@class="method has-srclink"]' "pub async fn mut_self(&mut self)"
90: @has check failed
 File does not exist 'async_fn/fn.named.html'
 // @has async_fn/fn.named.html
91: @has check failed
 File does not exist 'async_fn/fn.named.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn named<'a, 'b>(foo: &'a str) -> &'b str"
93: @has check failed
 File does not exist 'async_fn/fn.named_trait.html'
 // @has async_fn/fn.named_trait.html
94: @has check failed
 File does not exist 'async_fn/fn.named_trait.html'
 // @has - '//pre[@class="rust fn"]' "pub async fn named_trait<'a, 'b>(foo: impl Pattern<'a>) -> impl Pattern<'b>"
Encountered 31 errors

------------------------------------------



---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @has check failed
 File does not exist 'foo/primitive.i16.html'
 // @has 'foo/primitive.i16.html' '//h2[@id="synthetic-implementations"]' 'Auto Trait Implementation'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/all.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/all" "/checkout/src/test/rustdoc/all.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'foo/all.html'
 // @has foo/all.html '//a[@href="struct.Struct.html"]' 'Struct'
4: @has check failed
 File does not exist 'foo/all.html'
 // @has foo/all.html '//a[@href="enum.Enum.html"]' 'Enum'
5: @has check failed
 File does not exist 'foo/all.html'
 // @has foo/all.html '//a[@href="union.Union.html"]' 'Union'
6: @has check failed
 File does not exist 'foo/all.html'
 // @has foo/all.html '//a[@href="constant.CONST.html"]' 'CONST'
7: @has check failed
 File does not exist 'foo/all.html'
 // @has foo/all.html '//a[@href="static.STATIC.html"]' 'STATIC'
8: @has check failed
 File does not exist 'foo/all.html'
 // @has foo/all.html '//a[@href="fn.function.html"]' 'function'
26: @has check failed
 File does not exist 'foo/all.html'
 // @has foo/all.html '//a[@href="struct.ReexportedStruct.html"]' 'ReexportedStruct'
27: @!has check failed
 File does not exist 'foo/all.html'
 // @!has foo/all.html 'private_module'
Encountered 8 errors

------------------------------------------



---- [rustdoc] rustdoc/auto_aliases.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto_aliases" "/checkout/src/test/rustdoc/auto_aliases.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'auto_aliases/trait.Bar.html'
 // @has auto_aliases/trait.Bar.html '//div[@data-aliases="auto_aliases::Foo"]' 'impl Bar for Foo'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/assoc-types.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
3: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
 // @has assoc_types/trait.Index.html
5: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
         // @has - '//*[@id="associatedtype.Output"]//h4[@class="code-header"]' 'type Output: ?Sized'
7: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
     // @has - '//*[@id="tymethod.index"]//h4[@class="code-header"]' "fn index<'a>(&'a self, index: I) -> &'a Self::Output"
9: @has check failed
 File does not exist 'assoc_types/trait.Index.html'
     // @has - '//*[@id="tymethod.index"]//h4[@class="code-header"]//a[@href="trait.Index.html#associatedtype.Output"]' "Output"
14: @has check failed
 File does not exist 'assoc_types/fn.use_output.html'
 // @has assoc_types/fn.use_output.html
15: @has check failed
 File does not exist 'assoc_types/fn.use_output.html'
 // @has - '//*[@class="rust fn"]' '-> &T::Output'
16: @has check failed
 File does not exist 'assoc_types/fn.use_output.html'
 // @has - '//*[@class="rust fn"]//a[@href="trait.Index.html#associatedtype.Output"]' 'Output'
25: @has check failed
 File does not exist 'assoc_types/fn.use_input.html'
 // @has assoc_types/fn.use_input.html
26: @has check failed
 File does not exist 'assoc_types/fn.use_input.html'
 // @has - '//*[@class="rust fn"]' 'T::Input'
27: @has check failed
 File does not exist 'assoc_types/fn.use_input.html'
 // @has - '//*[@class="rust fn"]//a[@href="trait.Feed.html#associatedtype.Input"]' 'Input'
30: @has check failed
 File does not exist 'assoc_types/fn.cmp_input.html'
 // @has assoc_types/fn.cmp_input.html
31: @has check failed
 File does not exist 'assoc_types/fn.cmp_input.html'
 // @has - '//*[@class="rust fn"]' 'where T::Input: PartialEq<U::Input>'
32: @has check failed
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
 File does not exist 'assoc_types/fn.cmp_input.html'
 // @has - '//*[@class="rust fn"]//a[@href="trait.Feed.html#associatedtype.Input"]' 'Input'
Encountered 13 errors

------------------------------------------



---
test result: FAILED. 65 passed; 397 failed; 3 ignored; 0 measured; 0 filtered out; finished in 3.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:55
