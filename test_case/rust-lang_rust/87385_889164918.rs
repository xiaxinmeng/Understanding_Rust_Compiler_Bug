plain
-  --> $DIR/needless_borrow_pat.rs:60:14
+error: trailing semicolon in macro used in expression position
+  --> $DIR/needless_borrow_pat.rs:10:15
    |
-LL |         Some(ref x) => x,
-   |              ^^^^^ help: try this: `x`
-   |
-   = note: `-D clippy::needless-borrow` implied by `-D warnings`
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:66:14
-   |
-   |
-LL |         Some(ref x) => *x,
-   |              ^^^^^
-   |
-   |
-   |
-LL |         Some(x) => x,
-   |              ^     ^
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:72:14
-   |
-   |
-LL |         Some(ref x) => {
-   |              ^^^^^
-   |
-   |
-   |
-LL |         Some(x) => {
-LL |             f1(x);
-LL |             f1(x);
-   |
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:82:14
-   |
-   |
+LL |         f1($e);
+...
+...
 LL |         Some(ref x) => m1!(x),
-   |              ^^^^^ help: try this: `x`
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:87:15
+   |                        ------ in this macro invocation
    |
    |
-LL |     let _ = |&ref x: &&String| {
-   |               ^^^^^ help: try this: `x`
+   = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
+   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
+   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
+   = note: this error originates in the macro `m1` (in Nightly builds, run with -Z macro-backtrace for more info)
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:92:10
-   |
-   |
-LL |     let (ref y,) = (&x,);
-   |          ^^^^^
-   |
-   |
-   |
-LL |     let (y,) = (&x,);
-LL |     let _: &String = y;
-   |
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:102:14
-   |
-   |
-LL |         Some(ref x) => x.0,
-   |              ^^^^^ help: try this: `x`
-error: this pattern creates a reference to a reference
error: test failed, to rerun pass '--test compile-test'
-  --> $DIR/needless_borrow_pat.rs:112:14
-   |
-   |
-LL |         E::A(ref x) | E::B(ref x) => *x,
-   |              ^^^^^         ^^^^^
-   |
-   |
-   |
-LL |         E::A(x) | E::B(x) => x,
-   |              ^         ^     ^
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:118:21
-   |
-   |
-LL |         if let Some(ref x) = Some(&String::new());
-   |                     ^^^^^ help: try this: `x`
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:126:12
-   |
-   |
-LL | fn f2<'a>(&ref x: &&'a String) -> &'a String {
-   |            ^^^^^
-   |
-   |
-   |
-LL | fn f2<'a>(&x: &&'a String) -> &'a String {
-LL |     let _: &String = x;
-LL |     x
-   |
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:133:11
-   |
-   |
-LL |     fn f(&ref x: &&String) {
-   |           ^^^^^ help: try this: `x`
-error: this pattern creates a reference to a reference
-  --> $DIR/needless_borrow_pat.rs:141:11
-   |
-   |
-LL |     fn f(&ref x: &&String) {
-   |           ^^^^^
-   |
-   |
-   |
-LL |     fn f(&x: &&String) {
-LL |         let _: &String = x;
-   |
-error: aborting due to 12 previous errors
+error: aborting due to previous error
 
 
---
To only update this specific test, also pass `--test-args needless_borrow_pat.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/needless_borrow_pat.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_borrow_pat.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f67365d39a20a1e7.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/needless_borrow_pat.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"trailing semicolon in macro used in expression position","code":{"code":"semicolon_in_expressions_from_macros","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":224,"byte_end":225,"line_start":10,"line_end":10,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"        f1($e);","highlight_start":15,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":1616,"byte_end":1622,"line_start":82,"line_end":82,"column_start":24,"column_end":30,"is_primary":false,"text":[{"text":"        Some(ref x) => m1!(x),","highlight_start":24,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"m1!","def_site_span":{"file_name":"tests/ui/needless_borrow_pat.rs","byte_start":173,"byte_end":234,"line_start":8,"line_end":12,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! m1 {","highlight_start":1,"highlight_end":18},{"text":"    ($e:expr) => {","highlight_start":1,"highlight_end":19},{"text":"        f1($e);","highlight_start":1,"highlight_end":16},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D semicolon-in-expressions-from-macros` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing semicolon in macro used in expression position\n  --> tests/ui/needless_borrow_pat.rs:10:15\n   |\nLL |         f1($e);\n   |               ^\n...\nLL |         Some(ref x) => m1!(x),\n   |                        ------ in this macro invocation\n   |\n   = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>\n   = note: this error originates in the macro `m1` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: this pattern creates a reference to a reference
-  --> $DIR/ref_binding_to_reference.rs:31:14
+error: trailing semicolon in macro used in expression position
+  --> $DIR/ref_binding_to_reference.rs:10:16
    |
-LL |         Some(ref x) => x,
-   |              ^^^^^
-   |
-   = note: `-D clippy::ref-binding-to-reference` implied by `-D warnings`
-   |
-   |
-LL |         Some(x) => &x,
-   |              ^     ^^
-error: this pattern creates a reference to a reference
-  --> $DIR/ref_binding_to_reference.rs:37:14
-   |
-   |
-LL |         Some(ref x) => {
-   |              ^^^^^
-   |
-   |
-   |
-LL |         Some(x) => {
-LL |             f1(x);
-LL |             f1(x);
-LL |             &x
-   |
-error: this pattern creates a reference to a reference
-  --> $DIR/ref_binding_to_reference.rs:47:14
-   |
-   |
+LL |         f1(*$e);
+...
+...
 LL |         Some(ref x) => m2!(x),
-   |              ^^^^^
    |
-help: try this
-   |
-   |
-LL |         Some(x) => m2!(&x),
-   |              ^         ^^
+   = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`
+   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
+   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>
+   = note: this error originates in the macro `m2` (in Nightly builds, run with -Z macro-backtrace for more info)
-error: this pattern creates a reference to a reference
-  --> $DIR/ref_binding_to_reference.rs:52:15
-   |
-   |
-LL |     let _ = |&ref x: &&String| {
-   |               ^^^^^
-   |
-   |
-   |
-LL |     let _ = |&x: &&String| {
-LL |         let _: &&String = &x;
-   |
-error: this pattern creates a reference to a reference
-  --> $DIR/ref_binding_to_reference.rs:58:12
-   |
-   |
-LL | fn f2<'a>(&ref x: &&'a String) -> &'a String {
-   |            ^^^^^
-   |
-   |
-   |
-LL | fn f2<'a>(&x: &&'a String) -> &'a String {
-LL |     let _: &&String = &x;
-LL |     x
-   |
-error: this pattern creates a reference to a reference
-  --> $DIR/ref_binding_to_reference.rs:65:11
-   |
-   |
-LL |     fn f(&ref x: &&String) {
-   |           ^^^^^
-   |
-   |
-   |
-LL |     fn f(&x: &&String) {
-LL |         let _: &&String = &x;
-   |
-error: this pattern creates a reference to a reference
-  --> $DIR/ref_binding_to_reference.rs:73:11
-   |
-   |
-LL |     fn f(&ref x: &&String) {
-   |           ^^^^^
-   |
-   |
-   |
-LL |     fn f(&x: &&String) {
-LL |         let _: &&String = &x;
-   |
-error: aborting due to 7 previous errors
+error: aborting due to previous error
 
 
---
To only update this specific test, also pass `--test-args ref_binding_to_reference.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/ref_binding_to_reference.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/ref_binding_to_reference.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f67365d39a20a1e7.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-a05ee8fc41b4648a.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-54a7fffacad7d378.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-ff0575c5a2ca7cc8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7064d3f8532fb2a5.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/ref_binding_to_reference.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"trailing semicolon in macro used in expression position","code":{"code":"semicolon_in_expressions_from_macros","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/ref_binding_to_reference.rs","byte_start":234,"byte_end":235,"line_start":10,"line_end":10,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"        f1(*$e);","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/ref_binding_to_reference.rs","byte_start":911,"byte_end":917,"line_start":47,"line_end":47,"column_start":24,"column_end":30,"is_primary":false,"text":[{"text":"        Some(ref x) => m2!(x),","highlight_start":24,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"m2!","def_site_span":{"file_name":"tests/ui/ref_binding_to_reference.rs","byte_start":182,"byte_end":244,"line_start":8,"line_end":12,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! m2 {","highlight_start":1,"highlight_end":18},{"text":"    ($e:expr) => {","highlight_start":1,"highlight_end":19},{"text":"        f1(*$e);","highlight_start":1,"highlight_end":17},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D semicolon-in-expressions-from-macros` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: trailing semicolon in macro used in expression position\n  --> tests/ui/ref_binding_to_reference.rs:10:16\n   |\nLL |         f1(*$e);\n   |                ^\n...\nLL |         Some(ref x) => m2!(x),\n   |                        ------ in this macro invocation\n   |\n   = note: `-D semicolon-in-expressions-from-macros` implied by `-D warnings`\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #79813 <https://github.com/rust-lang/rust/issues/79813>\n   = note: this error originates in the macro `m2` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
