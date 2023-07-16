plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 552 tests
i...F.........F............F................................F..i........................ 88/552
................F..........................FFF......................F.F..........FF.F.FF 176/552
..FF.F.................................................................................. 264/552
.............F........F..F....F............F.....F.F..........................F........F 352/552
.....F................i....................F.F....FF.........FF......................... 440/552
......F...i.......F.......FF..F....F........F........................F.......F.F..F..... 528/552
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F.FF....................

---- [rustdoc] src/test/rustdoc/all.rs stdout ----


error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/all" "/checkout/src/test/rustdoc/all.rs"
stdout: none
--- stderr -------------------------------
27: Invalid number of @has arguments
 // @!has foo/all.html 'private_module'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/assoc-consts.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
stdout: none
--- stderr -------------------------------
8: Invalid number of @has arguments
     // @!has - FOO_HIDDEN
21: Invalid number of @has arguments
     // @!has - FOO_HIDDEN
53: Invalid number of @has arguments
     // @!has assoc_consts/struct.Bar.html 'BAR_PRIVATE'
55: Invalid number of @has arguments
     // @!has assoc_consts/struct.Bar.html 'BAR_HIDDEN'
Encountered 4 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/const-display.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-display" "/checkout/src/test/rustdoc/const-display.rs"
stdout: none
--- stderr -------------------------------
23: Invalid number of @has arguments
 // @!has - '//span[@class="since"]'
35: Invalid number of @has arguments
 // @!has - '//span[@class="since"]'
46: Invalid number of @has arguments
 // @!has - '//span[@class="since"]'
Encountered 3 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/deprecated-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-impls" "/checkout/src/test/rustdoc/deprecated-impls.rs"
stdout: none
--- stderr -------------------------------
78: Invalid number of @has arguments
     // @!has - 'fn_def_def_with_doc full'
89: Invalid number of @has arguments
     // @!has - 'fn_empty_with_doc full'
102: Invalid number of @has arguments
     // @!has - 'fn_def_with_doc full'
115: Invalid number of @has arguments
     // @!has - 'fn_def_def_with_doc full'
Encountered 4 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/empty-section.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-section" "/checkout/src/test/rustdoc/empty-section.rs"
stdout: none
--- stderr -------------------------------
8: Invalid number of @has arguments
 // @!has - 'Auto Trait Implementations'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/hidden-impls.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-impls" "/checkout/src/test/rustdoc/hidden-impls.rs"
stdout: none
--- stderr -------------------------------
14: Invalid number of @has arguments
 // @!has - 'Foo'
16: Invalid number of @has arguments
 // @!has - 'Foo'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/hidden-line.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-line" "/checkout/src/test/rustdoc/hidden-line.rs"
stdout: none
--- stderr -------------------------------
18: Invalid number of @has arguments
 // @!has hidden_line/fn.foo.html invisible
19: Tried to use the previous path in the first command
 // @matches - //pre "#\[derive\(PartialEq\)\] // Bar"
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/hidden-methods.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-methods" "/checkout/src/test/rustdoc/hidden-methods.rs"
stdout: none
--- stderr -------------------------------
20: Invalid number of @has arguments
 // @!has - 'Methods'
22: Invalid number of @has arguments
 // @!has - 'this_should_be_hidden'
26: Invalid number of @has arguments
 // @!has - 'Methods'
28: Invalid number of @has arguments
 // @!has - 'this_should_be_hidden'
Encountered 4 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_cross/assoc-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
stdout: none
--- stderr -------------------------------
10: Invalid number of @has arguments
 // @!has - 'PrivateConst'
13: Invalid number of @has arguments
 // @!has - 'private_method'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_cross/hidden-use.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/hidden-use" "/checkout/src/test/rustdoc/inline_cross/hidden-use.rs"
stdout: none
--- stderr -------------------------------
8: Invalid number of @has arguments
 // @!has - 'rustdoc_hidden'
9: Invalid number of @has arguments
 // @!has - 'Bar'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_local/glob-extern-document-private-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern-document-private-items" "/checkout/src/test/rustdoc/inline_local/glob-extern-document-private-items.rs"
stdout: none
--- stderr -------------------------------
17: Invalid number of @has arguments
 // @!has - "private_fn"
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_local/glob-extern.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern" "/checkout/src/test/rustdoc/inline_local/glob-extern.rs"
stdout: none
--- stderr -------------------------------
13: Invalid number of @has arguments
 // @!has - "mod1"
15: Invalid number of @has arguments
 // @!has - "private_fn"
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_local/glob-private-document-private-items.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-private-document-private-items" "/checkout/src/test/rustdoc/inline_local/glob-private-document-private-items.rs"
stdout: none
--- stderr -------------------------------
20: Invalid number of @has arguments
 // @!has - "Mod1Private"
21: Invalid number of @has arguments
 // @!has - "mod2"
23: Invalid number of @has arguments
 // @!has - "Mod2Private"
33: Invalid number of @has arguments
 // @!has - "Mod2Public"
34: Invalid number of @has arguments
 // @!has - "Mod2Private"
Encountered 5 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_local/hidden-use.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/hidden-use" "/checkout/src/test/rustdoc/inline_local/hidden-use.rs"
stdout: none
--- stderr -------------------------------
6: Invalid number of @has arguments
 // @!has - 'private'
7: Invalid number of @has arguments
 // @!has - 'Foo'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_local/glob-private.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-private" "/checkout/src/test/rustdoc/inline_local/glob-private.rs"
stdout: none
--- stderr -------------------------------
16: Invalid number of @has arguments
 // @!has - "mod1"
18: Invalid number of @has arguments
 // @!has - "Mod1Private"
19: Invalid number of @has arguments
 // @!has - "mod2"
21: Invalid number of @has arguments
 // @!has - "Mod2Private"
Encountered 4 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_local/macro_by_example.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/macro_by_example" "/checkout/src/test/rustdoc/inline_local/macro_by_example.rs"
stdout: none
--- stderr -------------------------------
10: Invalid number of @has arguments
     // @!has - 'pub use foo as bar;'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/inline_local/please_inline.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/please_inline" "/checkout/src/test/rustdoc/inline_local/please_inline.rs"
stdout: none
--- stderr -------------------------------
7: Invalid number of @has arguments
     // @!has - 'pub use foo::'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/internal.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/internal" "/checkout/src/test/rustdoc/internal.rs"
stdout: none
--- stderr -------------------------------
5: Invalid number of @matches arguments
 // @!matches internal/index.html '//*[@class="item-right docblock-short"]/span[@class="stab unstable"]'
7: Invalid number of @matches arguments
 // @!matches internal/index.html '//*[@class="item-right docblock-short"]/span[@class="stab internal"]'
9: Tried to use the previous path in the first command
 // @matches - '//*[@class="item-right docblock-short"]' 'Docs'
11: Invalid number of @has arguments
 // @!has internal/struct.S.html '//*[@class="stab unstable"]'
12: Invalid number of @has arguments
 // @!has internal/struct.S.html '//*[@class="stab internal"]'
Encountered 5 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-23812.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23812" "/checkout/src/test/rustdoc/issue-23812.rs"
stdout: none
--- stderr -------------------------------
20: Invalid number of @has arguments
 // @!has - '/// Outer comment'
22: Invalid number of @has arguments
 // @!has - '//! Inner comment'
34: Invalid number of @has arguments
 // @!has - '/** Outer block comment */'
36: Invalid number of @has arguments
 // @!has - '/*! Inner block comment */'
Encountered 4 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-27104.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-27104" "/checkout/src/test/rustdoc/issue-27104.rs"
stdout: none
--- stderr -------------------------------
6: Invalid number of @has arguments
 // @!has - 'extern crate std'
7: Invalid number of @has arguments
 // @!has - 'use std::prelude::'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-29584.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29584" "/checkout/src/test/rustdoc/issue-29584.rs"
stdout: none
--- stderr -------------------------------
7: Invalid number of @has arguments
 // @!has - 'impl Bar for'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-31899.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-31899" "/checkout/src/test/rustdoc/issue-31899.rs"
stdout: none
--- stderr -------------------------------
3: Invalid number of @has arguments
 // @!has - 'rust rust-example-rendered'
4: Invalid number of @has arguments
 // @!has - 'use ndarray::arr2'
5: Invalid number of @has arguments
 // @!has - 'prohibited'
Encountered 3 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-34473.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34473" "/checkout/src/test/rustdoc/issue-34473.rs"
stdout: none
--- stderr -------------------------------
8: Invalid number of @has arguments
 // @!has - SomeTypeWithLongName
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-32395.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32395" "/checkout/src/test/rustdoc/issue-32395.rs"
stdout: none
--- stderr -------------------------------
6: Invalid number of @has arguments
 // @!has - 'pub qux'
7: Invalid number of @has arguments
 // @!has - 'pub(crate) qux'
8: Invalid number of @has arguments
 // @!has - 'pub Bar'
12: Invalid number of @has arguments
 // @!has - 'pub qux'
13: Invalid number of @has arguments
 // @!has - 'pub(crate) qux'
14: Invalid number of @has arguments
 // @!has - 'pub Bar'
Encountered 6 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-41783.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-41783" "/checkout/src/test/rustdoc/issue-41783.rs"
stdout: none
--- stderr -------------------------------
2: Invalid number of @has arguments
 // @!has - 'space'
3: Invalid number of @has arguments
 // @!has - 'comment'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-53689.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-53689" "/checkout/src/test/rustdoc/issue-53689.rs"
stdout: none
--- stderr -------------------------------
8: Invalid number of @has arguments
 // @!has - 'MyStruct'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-61592.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-61592" "/checkout/src/test/rustdoc/issue-61592.rs"
stdout: none
--- stderr -------------------------------
8: Invalid number of @has arguments
 // @!has - '//a[@href="trait._.html"]'
14: Invalid number of @has arguments
 // @!has - '//a[@href="struct._.html"]'
Encountered 2 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/issue-89852.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-89852" "/checkout/src/test/rustdoc/issue-89852.rs"
stdout: none
--- stderr -------------------------------
7: Invalid number of @matches arguments
 // @!matches 'issue_89852/sidebar-items.js' '"repro".*"repro"'
Encountered 1 errors
------------------------------------------



---- [rustdoc] src/test/rustdoc/macro-private-not-documented.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/macro-private-not-documented" "/checkout/src/test/rustdoc/macro-private-not-documented.rs"
stdout: none
--- stderr -------------------------------
9: Invalid number of @has arguments
 // @!has macro_private_not_documented/index.html 'a_macro'
15: Invalid number of @has arguments
 // @!has macro_private_not_documented/index.html 'another_macro'
Encountered 2 errors
