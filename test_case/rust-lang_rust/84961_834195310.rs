plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
   Compiling cargo-util v0.1.0 (/checkout/src/tools/cargo/crates/cargo-util)
error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> src/tools/rustfmt/src/formatting.rs:37:9
    |
37  |           rustc_span::with_session_globals(self.config.edition().into(), || {
    |  _________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_----------------------------__-
    | |         expected 1 argument
    | |         expected 1 argument
38  | |             if self.config.disable_all_formatting() {
39  | |                 // When the input is from stdin, echo back the input.
40  | |                 if let Input::Text(ref buf) = input {
54  | |             })
55  | |         })
    | |_________- supplied 2 arguments
    |
    |
note: function defined here
   --> /checkout/compiler/rustc_span/src/lib.rs:138:8
    |
138 | pub fn with_session_globals<R, F>(f: F) -> R

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
---
   Compiling rustfmt-nightly v1.4.37 (/checkout/src/tools/rustfmt)
error[E0061]: this function takes 1 argument but 2 arguments were supplied
   --> src/tools/rustfmt/src/formatting.rs:37:9
    |
37  |           rustc_span::with_session_globals(self.config.edition().into(), || {
    |  _________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^_----------------------------__-
    | |         expected 1 argument
    | |         expected 1 argument
38  | |             if self.config.disable_all_formatting() {
39  | |                 // When the input is from stdin, echo back the input.
40  | |                 if let Input::Text(ref buf) = input {
54  | |             })
55  | |         })
    | |_________- supplied 2 arguments
    |
    |
note: function defined here
   --> /checkout/compiler/rustc_span/src/lib.rs:138:8
    |
138 | pub fn with_session_globals<R, F>(f: F) -> R

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
---

---- compile_test stdout ----
diff of stderr:

 error: you should put `foo_bar` between ticks in the documentation
    |
    |
 LL | /// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)
    |
    |
    = note: `-D clippy::doc-markdown` implied by `-D warnings`
 
 error: you should put `foo::bar` between ticks in the documentation
    |
    |
 LL | /// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)
 
 
 error: you should put `Foo::some_fun` between ticks in the documentation
    |
    |
 LL | /// Markdown is _weird_. I mean _really weird_. This /_ is ok. So is `_`. But not Foo::some_fun
 
 
 error: you should put `a::global:path` between ticks in the documentation
    |
    |
 LL | /// Here be ::a::global:path.
 
 
 error: you should put `NotInCodeBlock` between ticks in the documentation
    |
    |
 LL | /// That's not code ~NotInCodeBlock~.
 
 
 error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
    |
    |
 LL | /// be_sure_we_got_to_the_end_of_it
 
 
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+thread 'rustc' panicked at 'SESSION_GLOBALS should never be overwritten!', /checkout/compiler/rustc_span/src/lib.rs:104:5
 
 
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 
 
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 
 
-error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation
-   |
-   |
-LL | /// be_sure_we_got_to_the_end_of_it
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
 
-error: you should put `link_with_underscores` between ticks in the documentation
-   |
-   |
-LL | /// This test has [a link_with_underscores][chunked-example] inside it. See #823.
-   |                      ^^^^^^^^^^^^^^^^^^^^^
+note: Clippy version: clippy 0.1.53 (061d5fe 2021-05-07)
 
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
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
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
+query stack during panic:
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+end of query stack
 
 

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/doc.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doc.rs`
error: 1 errors occurred comparing output.
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/doc.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/doc.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/doc.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you should put `foo_bar` between ticks in the documentation","code":{"code":"clippy::doc_markdown","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc.rs","byte_start":244,"byte_end":251,"line_start":8,"line_end":8,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"/// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::doc-markdown` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: you should put `foo_bar` between ticks in the documentation\n  --> tests/ui/doc.rs:8:9\n   |\nLL | /// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)\n   |         ^^^^^^^\n   |\n   = note: `-D clippy::doc-markdown` implied by `-D warnings`\n\n"}
{"message":"you should put `foo::bar` between ticks in the documentation","code":{"code":"clippy::doc_markdown","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc.rs","byte_start":286,"byte_end":294,"line_start":8,"line_end":8,"column_start":51,"column_end":59,"is_primary":true,"text":[{"text":"/// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)","highlight_start":51,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: you should put `foo::bar` between ticks in the documentation\n  --> tests/ui/doc.rs:8:51\n   |\nLL | /// The foo_bar function does _nothing_. See also foo::bar. (note the dot there)\n   |                                                   ^^^^^^^^\n\n"}
{"message":"you should put `Foo::some_fun` between ticks in the documentation","code":{"code":"clippy::doc_markdown","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc.rs","byte_start":399,"byte_end":412,"line_start":9,"line_end":9,"column_start":83,"column_end":96,"is_primary":true,"text":[{"text":"/// Markdown is _weird_. I mean _really weird_. This \\_ is ok. So is `_`. But not Foo::some_fun","highlight_start":83,"highlight_end":96}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: you should put `Foo::some_fun` between ticks in the documentation\n  --> tests/ui/doc.rs:9:83\n   |\nLL | /// Markdown is _weird_. I mean _really weird_. This \\_ is ok. So is `_`. But not Foo::some_fun\n   |                                                                                   ^^^^^^^^^^^^^\n\n"}
{"message":"you should put `a::global:path` between ticks in the documentation","code":{"code":"clippy::doc_markdown","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc.rs","byte_start":496,"byte_end":510,"line_start":11,"line_end":11,"column_start":15,"column_end":29,"is_primary":true,"text":[{"text":"/// Here be ::a::global:path.","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: you should put `a::global:path` between ticks in the documentation\n  --> tests/ui/doc.rs:11:15\n   |\nLL | /// Here be ::a::global:path.\n   |               ^^^^^^^^^^^^^^\n\n"}
{"message":"you should put `NotInCodeBlock` between ticks in the documentation","code":{"code":"clippy::doc_markdown","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc.rs","byte_start":533,"byte_end":547,"line_start":12,"line_end":12,"column_start":22,"column_end":36,"is_primary":true,"text":[{"text":"/// That's not code ~NotInCodeBlock~.","highlight_start":22,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: you should put `NotInCodeBlock` between ticks in the documentation\n  --> tests/ui/doc.rs:12:22\n   |\nLL | /// That's not code ~NotInCodeBlock~.\n   |                      ^^^^^^^^^^^^^^\n\n"}
{"message":"you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation","code":{"code":"clippy::doc_markdown","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc.rs","byte_start":554,"byte_end":585,"line_start":13,"line_end":13,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"/// be_sure_we_got_to_the_end_of_it","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: you should put `be_sure_we_got_to_the_end_of_it` between ticks in the documentation\n  --> tests/ui/doc.rs:13:5\n   |\nLL | /// be_sure_we_got_to_the_end_of_it\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
thread 'rustc' panicked at 'SESSION_GLOBALS should never be overwritten!', /checkout/compiler/rustc_span/src/lib.rs:104:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.53 (061d5fe 2021-05-07)
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
{"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}

------------------------------------------


diff of stderr:

-error: needless `fn main` in doctest
-  --> $DIR/needless_doc_main.rs:7:4
-   |
-LL | /// fn main() {
-   |    ^^^^^^^^^^^^
-   |
-   = note: `-D clippy::needless-doctest-main` implied by `-D warnings`
+thread 'rustc' panicked at 'SESSION_GLOBALS should never be overwritten!', /checkout/compiler/rustc_span/src/lib.rs:104:5
 
 
-error: needless `fn main` in doctest
-  --> $DIR/needless_doc_main.rs:14:4
-   |
-LL | /// fn main() -> () {
-   |    ^^^^^^^^^^^^^^^^^^
 
 
-error: needless `fn main` in doctest
-  --> $DIR/needless_doc_main.rs:21:4
-   |
-LL | /// fn main() {
-   |    ^^^^^^^^^^^^
 
 
-error: needless `fn main` in doctest
-  --> $DIR/needless_doc_main.rs:28:4
-   |
-LL | /// fn main() {
-   |    ^^^^^^^^^^^^
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
-error: aborting due to 4 previous errors
-error: aborting due to 4 previous errors
+note: Clippy version: clippy 0.1.53 (061d5fe 2021-05-07)
+query stack during panic:
+query stack during panic:
+#0 [analysis] running analysis passes on this crate
+end of query stack

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_doc_main.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args needless_doc_main.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_doc_main.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_doc_main.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_doc_main.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'SESSION_GLOBALS should never be overwritten!', /checkout/compiler/rustc_span/src/lib.rs:104:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.53 (061d5fe 2021-05-07)
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
