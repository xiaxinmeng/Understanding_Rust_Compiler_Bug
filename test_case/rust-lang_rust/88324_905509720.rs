plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9863bf51a52b8e61bcad312f81b5193d53099f9f and 7b8b5a8eb7a367bd0c2b296dc453377b2d93d6d9
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

-error: you should put `foo_bar` between ticks in the documentation
+error: constant expression depends on a generic parameter
+  --> $DIR/doc.rs:208:7
    |
    |
-LL | /// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)
-   |         ^^^^^^^
+LL | where [(); N.checked_next_power_of_two().unwrap()]: {
    |
    |
-   = note: `-D clippy::doc-markdown` implied by `-D warnings`
+   = note: this may fail depending on what value the parameter takes
 
-error: you should put `foo::bar` between ticks in the documentation
+error: constant expression depends on a generic parameter
+  --> $DIR/doc.rs:209:10
    |
    |
-LL | /// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)
-   |                                                   ^^^^^^^^
-
-error: you should put `Foo::some_fun` between ticks in the documentation
-  --> $DIR/doc.rs:9:83
+LL |     arr: [T; N.checked_next_power_of_two().unwrap()],
error: test failed, to rerun pass '--test compile-test'
    |
    |
-LL | /// Markdown is _weird_. I mean _really weird_. This /_ is ok. So is `_`. But not Foo::some_fun
-   |                                                                                   ^^^^^^^^^^^^^
+   = note: this may fail depending on what value the parameter takes
 
-error: you should put `a::global:path` between ticks in the documentation
+error: constant expression depends on a generic parameter
+  --> $DIR/doc.rs:214:7
    |
    |
-LL | /// Here be ::a::global:path.
-   |               ^^^^^^^^^^^^^^
-
-error: you should put `NotInCodeBlock` between ticks in the documentation
-  --> $DIR/doc.rs:12:22
+LL | where [(); N.checked_next_power_of_two().unwrap()]: {
    |
    |
-LL | /// That's not code ~NotInCodeBlock~.
-   |                      ^^^^^^^^^^^^^^
+   = note: this may fail depending on what value the parameter takes
 
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `link_with_underscores` between ticks in the documentation
-   |
-   |
-LL | /// This test has [a link_with_underscores][chunked-example] inside it. See #823.
-   |                      ^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `inline_link2` between ticks in the documentation
-   |
-   |
-LL | /// It can also be [inline_link2].
-   |                     ^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `CamelCaseThing` between ticks in the documentation
-   |
-   |
-LL | /// ## CamelCaseThing
-   |        ^^^^^^^^^^^^^^
-
-error: you should put `CamelCaseThing` between ticks in the documentation
-   |
-   |
-LL | /// # CamelCaseThing
-   |       ^^^^^^^^^^^^^^
-
-error: you should put `CamelCaseThing` between ticks in the documentation
-   |
-   |
-LL | /// Not a title #897 CamelCaseThing
-   |                      ^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `FooBar` between ticks in the documentation
-   |
-   |
-LL | /** E.g., serialization of an empty list: FooBar
-   |                                           ^^^^^^
-
-error: you should put `BarQuz` between ticks in the documentation
-   |
-   |
-LL | And BarQuz too.
-   |     ^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | be_sure_we_got_to_the_end_of_it
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `FooBar` between ticks in the documentation
-   |
-   |
-LL | /** E.g., serialization of an empty list: FooBar
-   |                                           ^^^^^^
-
-error: you should put `BarQuz` between ticks in the documentation
-   |
-   |
-LL | And BarQuz too.
-   |     ^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | be_sure_we_got_to_the_end_of_it
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put bare URLs between `<`/`>` or make a proper Markdown link
-   |
-   |
-LL | /// Not ok: http://www.unicode.org
-   |             ^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put bare URLs between `<`/`>` or make a proper Markdown link
-   |
-   |
-LL | /// Not ok: https://www.unicode.org
-   |             ^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put bare URLs between `<`/`>` or make a proper Markdown link
-   |
-   |
-LL | /// Not ok: http://www.unicode.org/
-   |             ^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put bare URLs between `<`/`>` or make a proper Markdown link
-   |
-   |
-LL | /// Not ok: http://www.unicode.org/reports/tr9/#Reordering_Resolved_Levels
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-
-error: you should put `mycrate::Collection` between ticks in the documentation
-   |
-   |
-LL | /// An iterator over mycrate::Collection's values.
-   |                      ^^^^^^^^^^^^^^^^^^^
-error: aborting due to 31 previous errors
+error: aborting due to 3 previous errors
 
 
---
To only update this specific test, also pass `--test-args doc/doc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/doc/doc.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/doc/doc.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-06539012b522c0cf.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8e5c09d8dc991112.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-90129c5dba212efa.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-17d0421109b348d4.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/doc/doc.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"constant expression depends on a generic parameter","code":null,"level":"error","spans":[{"file_name":"tests/ui/doc/doc.rs","byte_start":5458,"byte_end":5502,"line_start":208,"line_end":208,"column_start":7,"column_end":51,"is_primary":true,"text":[{"text":"where [(); N.checked_next_power_of_two().unwrap()]: {","highlight_start":7,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may fail depending on what value the parameter takes","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: constant expression depends on a generic parameter\n  --> tests/ui/doc/doc.rs:208:7\n   |\nLL | where [(); N.checked_next_power_of_two().unwrap()]: {\n   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this may fail depending on what value the parameter takes\n\n"}
{"message":"constant expression depends on a generic parameter","code":null,"level":"error","spans":[{"file_name":"tests/ui/doc/doc.rs","byte_start":5515,"byte_end":5558,"line_start":209,"line_end":209,"column_start":10,"column_end":53,"is_primary":true,"text":[{"text":"    arr: [T; N.checked_next_power_of_two().unwrap()],","highlight_start":10,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may fail depending on what value the parameter takes","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: constant expression depends on a generic parameter\n  --> tests/ui/doc/doc.rs:209:10\n   |\nLL |     arr: [T; N.checked_next_power_of_two().unwrap()],\n   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this may fail depending on what value the parameter takes\n\n"}
{"message":"constant expression depends on a generic parameter","code":null,"level":"error","spans":[{"file_name":"tests/ui/doc/doc.rs","byte_start":5631,"byte_end":5675,"line_start":214,"line_end":214,"column_start":7,"column_end":51,"is_primary":true,"text":[{"text":"where [(); N.checked_next_power_of_two().unwrap()]: {","highlight_start":7,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may fail depending on what value the parameter takes","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: constant expression depends on a generic parameter\n  --> tests/ui/doc/doc.rs:214:7\n   |\nLL | where [(); N.checked_next_power_of_two().unwrap()]: {\n   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this may fail depending on what value the parameter takes\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
