plain
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 136 tests
.................................................................................................... 100/136
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F..................F................

---- [ui] rustdoc-ui/invalid-html-tags.rs stdout ----
diff of stderr:


- error: unclosed HTML tag `p`
-   --> $DIR/invalid-html-tags.rs:3:5
+ error: unclosed HTML tag `img`
3    |
3    |
- LL | //! <p>💩<p>
-    |     ^^^
+ LL | /// <img><input>
6    |
7 note: the lint level is defined here
8   --> $DIR/invalid-html-tags.rs:1:9


10 LL | #![deny(rustdoc::invalid_html_tags)]
12 
12 
- error: unclosed HTML tag `p`
-   --> $DIR/invalid-html-tags.rs:3:9
+ error: unclosed HTML tag `input`
15    |
15    |
- LL | //! <p>💩<p>
-    |          ^^^
+ LL | /// <img><input>
18 
18 
- error: unclosed HTML tag `unknown`
-   --> $DIR/invalid-html-tags.rs:11:5
-    |
- LL | /// <unknown>
- 
- 
- error: unclosed HTML tag `script`
-   --> $DIR/invalid-html-tags.rs:14:5
-    |
- LL | /// <script>
- 
- 
31 error: unclosed HTML tag `h2`
33    |


52 LL | ///    <br/> <p>
54 
54 
- error: unclosed HTML tag `div`
-   --> $DIR/invalid-html-tags.rs:41:5
-    |
- LL | /// <div style="hello">
- 
- 
- error: unclosed HTML tag `h3`
-   --> $DIR/invalid-html-tags.rs:43:7
-    |
- LL | ///   <h3>
- 
- 
- error: unclosed HTML tag `script`
-   --> $DIR/invalid-html-tags.rs:45:5
-    |
- LL | /// <script
- 
- 
- error: unclosed HTML tag `div`
-   --> $DIR/invalid-html-tags.rs:73:5
-    |
- LL | /// <div></div
- 
79 error: Unclosed HTML comment
80   --> $DIR/invalid-html-tags.rs:87:5
81    |
---
To only update this specific test, also pass `--test-args invalid-html-tags.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-html-tags.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-html-tags" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-html-tags/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unclosed HTML tag `img`
   |
   |
LL | /// <img><input>
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/invalid-html-tags.rs:1:9
   |
   |
LL | #![deny(rustdoc::invalid_html_tags)]


error: unclosed HTML tag `input`
   |
   |
LL | /// <img><input>


error: unclosed HTML tag `h2`
   |
LL | ///   <h2>
   |       ^^^^


error: unclosed HTML tag `h3`
   |
LL | ///     <h3>
   |         ^^^^


error: unopened HTML tag `hello`
   |
   |
LL | /// </hello>


error: unclosed HTML tag `p`
   |
   |
LL | ///    <br/> <p>

error: Unclosed HTML comment
  --> /checkout/src/test/rustdoc-ui/invalid-html-tags.rs:87:5
   |
---

---- [ui] rustdoc-ui/lint-group.rs stdout ----
diff of stderr:

48    = note: `#[deny(rustdoc::broken_intra_doc_links)]` implied by `#[deny(rustdoc::all)]`
49    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
50 
- error: unclosed HTML tag `unknown`
-   --> $DIR/lint-group.rs:26:5
-    |
- LL | /// <unknown>
-    |
- note: the lint level is defined here
-   --> $DIR/lint-group.rs:7:9
-    |
-    |
- LL | #![deny(rustdoc::all)]
-    |         ^^^^^^^^^^^^
-    = note: `#[deny(rustdoc::invalid_html_tags)]` implied by `#[deny(rustdoc::all)]`
- error: aborting due to 5 previous errors
+ error: aborting due to 4 previous errors
65 
66 
---
To only update this specific test, also pass `--test-args lint-group.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/lint-group.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-group" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-group/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: missing code example in this documentation
  --> /checkout/src/test/rustdoc-ui/lint-group.rs:16:1
   |
LL | /// wait, this doesn't have a doctest?
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/lint-group.rs:7:9
   |
   |
LL | #![deny(rustdoc::all)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(rustdoc::missing_doc_code_examples)]` implied by `#[deny(rustdoc::all)]`
error: documentation test in private item
  --> /checkout/src/test/rustdoc-ui/lint-group.rs:19:1
   |
   |
LL | / /// wait, this *does* have a doctest?
LL | | ///
LL | | /// 